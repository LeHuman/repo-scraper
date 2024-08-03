use std::collections::{BTreeMap, BTreeSet};

use crate::reposcrape::{Project, Repo};

use super::repo::RepoScrapeCache;

#[derive(Eq, PartialEq, Default, Debug)]
pub struct ExpandedRepoCache<'c> {
    pub repos: BTreeMap<&'c String, &'c Repo>,
    pub single_repos: BTreeSet<&'c Repo>,
    pub projects: BTreeMap<&'c String, Project<'c>>,
}

impl<'c> ExpandedRepoCache<'c> {
    pub fn new(cache: &'c RepoScrapeCache) -> ExpandedRepoCache<'c> {
        let mut expanded = ExpandedRepoCache::default();

        if cache.is_empty() {
            return expanded;
        }

        expanded.repos = cache
            .repos
            .data
            .iter()
            .map(|repo| (&repo.uid, repo))
            .collect();

        for repo in &cache.repos.data {
            let mut single_repo = true;
            if let Some(details) = &repo.details {
                if let Some(project_name) = &details.project {
                    let repo_name = match &details.title {
                        Some(title) => title,
                        None => &repo.name,
                    };
                    let main_repo = match &details.main {
                        Some(_) => true,
                        None => project_name == repo_name,
                    };

                    let entry = expanded.projects.entry(project_name).or_insert(Project {
                        name: project_name,
                        description: None,
                        repo_main: None,
                        repo_sub: BTreeSet::new(),
                    });

                    // Main repo for this project
                    // NOTE: main repo for projects is optional
                    // NOTE: priority is given to first found repo that has main defined, in any case
                    if main_repo {
                        let mut existing_main = false;
                        // TODO: Warn that repo main was already set
                        if let Some(existing) = entry.repo_main {
                            if let Some(existing_details) = &existing.details {
                                existing_main = existing_details.main.is_some();
                                if !existing_main {
                                    entry.repo_sub.insert(existing);
                                }
                            } else {
                                entry.repo_sub.insert(existing);
                            }
                        }
                        if existing_main {
                            entry.repo_sub.insert(repo);
                        } else {
                            entry.repo_main = Some(repo);
                            entry.description = details.description.as_ref();
                        }
                    } else {
                        entry.repo_sub.insert(repo);
                    }
                    single_repo = false;
                }
            }
            if single_repo {
                expanded.single_repos.insert(repo);
            }
        }

        let mut to_remove: Vec<&String> = Vec::new();

        // NOTE: projects with single repositories (including main) will be treated as a single repo instead
        for (key, project) in &expanded.projects {
            if let Some(repo) = project.get_single() {
                to_remove.push(key);
                expanded.single_repos.insert(repo);
            }
        }

        for key in to_remove {
            expanded.projects.remove(key);
        }

        expanded
    }
}
