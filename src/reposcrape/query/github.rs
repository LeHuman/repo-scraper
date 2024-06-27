use std::collections::BTreeSet;

use octocrab::Octocrab;

use crate::{
    date::{Epoch, EpochType},
    reposcrape::{Metadata, Repo},
};

use super::query::{QueryInterface, QueryResult};

pub struct GHQuery {
    octocrab: Octocrab,
}

const ORIGIN: &str = "GitHub";

impl GHQuery {
    pub fn new(instance: Octocrab) -> Self {
        Self { octocrab: instance }
    }

    fn form_qstr(raw_query: String) -> serde_json::Value {
        return serde_json::json!({ "query": raw_query });
    }

    fn qstr_latest(username: &str, max_count: u32) -> String {
        format!(
            r#"query {{
        user(login: "{username}") {{
            repositories(first: {max_count}, orderBy: {{ field: UPDATED_AT, direction: DESC }}) {{
                nodes {{
                    id
                    name
                    updatedAt
                    owner{{login}}
                    object(expression: "HEAD:README.md") {{
                        ... on Blob {{
                            text
                        }}
                    }}
                }}
            }}
        }}
    }}"#
        )
    }

    fn qstr_dated(username: &str, max_count: u32, after_epoch: EpochType) -> String {
        let local_date = match Epoch::to_rfc3339(after_epoch) {
            Some(s) => s,
            None => {
                println!("Failed to parse rfc3339 from epoch, defaulting to zero");
                "1970-01-01T00:00:00Z".to_owned()
            }
        };
        format!(
            r#"query {{
    search(type: REPOSITORY, first: {max_count}, query: "user:{username} pushed:>{local_date}") {{
        nodes {{
        ... on Repository {{
            id
            name
            updatedAt
            owner{{login}}
            object(expression: "HEAD:README.md") {{
            ... on Blob {{
                text
                    }}
                }}
            }}
        }}
    }}
}}"#
        )
    }

    async fn process_response_node(
        repo_val: &serde_json::Value,
        today_epoch: EpochType,
    ) -> Option<Repo> {
        let id = repo_val["id"].as_str()?.to_owned();
        let name: String = repo_val["name"].as_str()?.to_owned();
        let updated_at = repo_val["updatedAt"].as_str()?.to_owned();
        let updated_at = match Epoch::from_rfc3339(&updated_at) {
            Ok(e) => e,
            Err(_) => {
                println!("Failed to parse repo update time");
                0
            }
        };
        let owner = repo_val["owner"].as_object()?["login"].as_str()?.to_owned();
        let readme_text = repo_val["object"].as_object()?["text"].as_str()?; // NOTE: fn ignores repositories with no README.md
        let metadata = Metadata::extract(readme_text);
        Some(Repo::new(
            id,
            name,
            owner,
            ORIGIN.to_owned(),
            today_epoch,
            updated_at,
            &metadata,
        ))
    }

    fn get_response_nodes(json_value: &serde_json::Value) -> Option<&serde_json::Value> {
        match json_value {
            serde_json::Value::Object(map) => match map.get("nodes") {
                Some(nodes) => return Some(nodes),
                None => {
                    for (_, value) in map.iter() {
                        if let Some(result) = Self::get_response_nodes(value) {
                            return Some(result);
                        }
                    }
                }
            },
            serde_json::Value::Array(array) => {
                // Traverse each element in the array recursively
                for element in array.iter() {
                    if let Some(result) = Self::get_response_nodes(element) {
                        return Some(result);
                    }
                }
            }
            _ => return None,
        }
        None
    }

    async fn call_query(&self, query: &serde_json::Value) -> QueryResult {
        let response: serde_json::Value = self.octocrab.graphql(query).await?;

        let response_nodes = match match Self::get_response_nodes(&response) {
            Some(val) => val.as_array(),
            None => return Err(Box::from("Failed to find nodes from query response")),
        } {
            Some(arr) => arr,
            None => {
                return Err(Box::from(
                    "Failed to access nodes as array from query response",
                ))
            }
        };

        let now_epoch = Epoch::get_local();

        let mut node_process = Vec::new();

        for repo_node in response_nodes {
            node_process.push(Self::process_response_node(repo_node, now_epoch));
        }

        let mut result: BTreeSet<Repo> = BTreeSet::new();

        for node in node_process {
            match node.await {
                Some(repo) => result.insert(repo),
                None => continue,
            };
        }

        Ok(result)
    }
}

impl QueryInterface for GHQuery {
    async fn fetch_latest(&self, user: &str, max_count: u32) -> QueryResult {
        self.call_query(&Self::form_qstr(Self::qstr_latest(user, max_count)))
            .await
    }

    async fn fetch_after(&self, user: &str, max_count: u32, after_epoch: EpochType) -> QueryResult {
        self.call_query(&Self::form_qstr(Self::qstr_dated(
            user,
            max_count,
            after_epoch,
        )))
        .await
    }
}
