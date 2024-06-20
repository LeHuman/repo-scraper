use super::repo::Repo;

use bincode::{Decode, Encode};
use std::collections::{BTreeMap, BTreeSet};

pub struct Project<'p> {
    name: &'p String,
    description: &'p String,
    repo_main: &'p Repo,
    repo_sub: BTreeSet<&'p Repo>,
}

#[derive(Eq, PartialEq, Encode, Decode)]
pub struct ProjectEntry {
    name: String,
    description: String,
    repo_id_main: String,
    repo_id_sub: BTreeSet<String>,
}

impl ProjectEntry {
    fn new(main: Repo) -> ProjectEntry {
        let details = main.details.unwrap(); // TODO: ensure given repo has details defined
        return ProjectEntry {
            name: details.project.unwrap(),
            description: details.description.unwrap(),
            repo_id_main: main.id,
            repo_id_sub: BTreeSet::new(),
        };
    }

    fn add(&mut self, repo: Repo) {
        self.repo_id_sub.insert(repo.id);
    }

    fn resolve<'r>(&'r self, repos: &'r BTreeMap<String, Repo>) -> Option<Project> {
        let main: &'r Repo = repos.get(&self.repo_id_main)?;
        let mut repo_sub: BTreeSet<&'r Repo> = BTreeSet::new();

        for repo_id in &self.repo_id_sub {
            repo_sub.insert(repos.get(repo_id)?);
        }

        let proj = Project {
            name: &self.name,
            description: &self.description,
            repo_main: main,
            repo_sub: repo_sub,
        };

        return Some(proj);
    }
}
