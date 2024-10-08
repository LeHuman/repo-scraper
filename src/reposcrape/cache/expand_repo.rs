use std::collections::{BTreeMap, BTreeSet};

use tracing::warn;

use crate::reposcrape::{metadata::extract_urls, Project, Repo};

use super::repo::RepoScrapeCache;

#[derive(Eq, PartialEq, Default, Debug)]
pub struct ExpandedRepoCache {
    pub repos: BTreeMap<String, Repo>,
    pub projects: BTreeMap<String, Project>,
}

impl ExpandedRepoCache {
    pub async fn new(mut cache: RepoScrapeCache) -> ExpandedRepoCache {
        let mut expanded = ExpandedRepoCache::default();

        if cache.is_empty() {
            return expanded;
        }

        while !cache.repos.data.is_empty() {
            let Some(repo) = cache.repos.data.pop_last() else {
                continue;
            };
            let Some(details) = &repo.details else {
                expanded.repos.insert(repo.uid.to_owned(), repo);
                continue;
            };
            let Some(project_name) = details.project.clone() else {
                expanded.repos.insert(repo.uid.to_owned(), repo);
                continue;
            };
            let repo_name = match &details.title {
                Some(title) => title,
                None => &repo.name,
            };
            let mut main_repo = match &details.main {
                Some(_) => true,
                None => &project_name == repo_name,
            };
            let mut project = match expanded.projects.remove(&project_name.to_owned()) {
                Some(entry) => entry,
                None => Project {
                    name: project_name.to_owned(),
                    description: None,
                    repo_main: None,
                    repo_sub: BTreeSet::new(),
                },
            };

            // NOTE: priority is given to first found repo that has main defined, in any case
            if main_repo && project.repo_main.is_some() {
                warn!("Project identified multiple mains {}", &project_name);
                if let Some(repo_main) = &project.repo_main {
                    if let Some(details) = &repo_main.details {
                        match details.main {
                            Some(_) => {
                                main_repo = false;
                            }
                            None => {
                                let repo = project.repo_main;
                                project.repo_main = None;
                                if let Some(repo) = repo {
                                    project.repo_sub.insert(repo);
                                }
                            }
                        }
                    }
                }
            }

            // Main repo for this project
            // NOTE: main repo for projects is optional
            if main_repo {
                details.description.clone_into(&mut project.description);
                project.repo_main = Some(repo);
            } else {
                project.repo_sub.insert(repo);
            }

            expanded.projects.insert(project_name.to_owned(), project);
        }

        let client = reqwest::Client::new();

        for project in expanded.projects.values() {
            let Some(repo) = &project.repo_main else {
                continue;
            };
            let Some(details) = &repo.details else {
                continue;
            };
            let Some(children) = &details.children else {
                continue;
            };

            // NOTE: Children stored as urls on individual lines, with the way the parser works, it is expected this is a single string containing all of them
            let extracted_urls = extract_urls(&children.lines().collect());
            if extracted_urls.len() == project.repo_sub.len() {
                // TODO: Option to not skip project if count matches?
                continue;
            }

            warn!("Searching project {} child urls", project.name);

            let mut repo_urls = Vec::new();

            // TODO: check response status?
            for repo in &project.repo_sub {
                if let Ok(resp) = client.get(&repo.url).send().await {
                    repo_urls.push(resp.url().to_owned());
                }
            }

            for child_url in extracted_urls {
                if let Ok(resp) = client.get(child_url).send().await {
                    let final_url = resp.url();
                    if !repo_urls.contains(final_url) {
                        warn!("Requesting child URL that was not listed {}", &final_url);
                        // TODO: Request Repo object from child url 'final_url'
                    }
                }
            }
        }

        let mut to_remove: Vec<String> = Vec::new();

        // TODO: Option to keep projects with just a main?
        // NOTE: projects with single repositories (including one with main) will be treated as a single repo instead
        for (key, project) in &expanded.projects {
            if project.is_single() {
                to_remove.push(key.to_owned());
            }
        }

        for key in to_remove {
            let Some(project) = expanded.projects.remove(&key) else {
                continue;
            };
            let Some(repo) = project.get_single() else {
                continue;
            };
            expanded.repos.insert(repo.uid.to_owned(), repo);
        }

        expanded
    }
}
