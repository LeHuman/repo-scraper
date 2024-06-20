use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use octocrab::Octocrab;

use crate::reposcrape::Date;
use crate::reposcrape::{repo::Repo, Metadata};

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

    fn qstr_dated(username: &str, max_count: u32, after_date: DateTime<Utc>) -> String {
        let local_date = Date::to_date_str(after_date);
        format!(
            r#"query {{
    search(type: REPOSITORY, first: {max_count}, query: "user:{username} pushed:>{local_date}") {{
        nodes {{
        ... on Repository {{
            id
            name
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
        today_str: &String,
    ) -> Option<Repo> {
        let id = repo_val["id"].as_str()?.to_owned();
        let name = repo_val["name"].as_str()?.to_owned();
        let owner = repo_val["owner"].as_object()?["login"].as_str()?.to_owned();
        let readme_text = repo_val["object"].as_object()?["text"].as_str()?; // NOTE: fn ignores repositories with no README.md
        let metadata = Metadata::extract(readme_text);
        Some(Repo::new(
            id,
            name,
            owner,
            ORIGIN.to_owned(),
            today_str.to_owned(),
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

        let now_str = Date::get_local_date_str();

        let mut node_process = Vec::new();

        for repo_node in response_nodes {
            node_process.push(Self::process_response_node(repo_node, &now_str));
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

    async fn fetch_after(
        &self,
        user: &str,
        max_count: u32,
        after_date: DateTime<Utc>,
    ) -> QueryResult {
        self.call_query(&Self::form_qstr(Self::qstr_dated(
            user, max_count, after_date,
        )))
        .await
    }
}
