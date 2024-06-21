use super::repo::Repo;

use std::collections::BTreeSet;

#[derive(Eq, PartialEq)]
pub struct Project<'p> {
    pub name: &'p String,
    pub description: Option<&'p String>,
    pub repo_main: Option<&'p Repo>,
    pub repo_sub: BTreeSet<&'p Repo>,
}

impl<'p> Project<'p> {
    pub fn get_single(&self) -> Option<&'p Repo> {
        let has_main = self.repo_main.is_some();
        match has_main as usize + self.repo_sub.len() {
            1 => {
                if has_main {
                    self.repo_main
                } else {
                    self.repo_sub.last().copied()
                }
            }
            _ => None,
        }
    }
}
