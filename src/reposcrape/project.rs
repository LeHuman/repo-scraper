use super::Repo;

use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub repo_main: Option<Repo>,
    pub repo_sub: BTreeSet<Repo>,
}

impl Project {
    pub fn get_single(mut self) -> Option<Repo> {
        let has_main = self.repo_main.is_some();
        if (has_main as usize + self.repo_sub.len()) == 1 {
            if has_main {
                self.repo_main
            } else {
                self.repo_sub.pop_last()
            }
        } else {
            None
        }
    }
    pub fn is_single(&self) -> bool {
        ((self.repo_main.is_some() as usize) + self.repo_sub.len()) == 1
    }
}
