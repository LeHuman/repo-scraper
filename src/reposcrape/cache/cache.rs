use std::collections::{BTreeSet, HashMap};

use bincode::{Decode, Encode};

use crate::{
    cache::{Cachable, Cache, Update},
    date::Epoch,
    reposcrape::Repo,
};

#[derive(Eq, PartialEq, Encode, Decode)]
pub struct RepoScrapeCache {
    pub repos: Cachable<BTreeSet<Repo>>,
    pub colors: Cachable<HashMap<String, String>>,
}

impl Default for RepoScrapeCache {
    fn default() -> Self {
        let mut ret = Self {
            repos: Default::default(),
            colors: Default::default(),
        };
        // NOTE: TTL is currently hardcoded for cache, must be manually deleted if updated
        ret.repos.days_to_update = 14;
        ret.colors.days_to_update = 60;
        ret
    }
}

impl RepoScrapeCache {
    pub fn new(
        repos: Option<Cachable<BTreeSet<Repo>>>,
        colors: Option<Cachable<HashMap<String, String>>>,
    ) -> Self {
        Self {
            repos: repos.unwrap_or_default(),
            colors: colors.unwrap_or_default(),
        }
    }
}

impl Cache for RepoScrapeCache {
    fn is_empty(&self) -> bool {
        self.repos.data.is_empty()
    }
}

impl Update<BTreeSet<Repo>> for Cachable<BTreeSet<Repo>> {
    fn update(&mut self, other: &BTreeSet<Repo>) {
        // TODO: Test if extending on incoming repos changes anything or if persistance depends on Ord impl
        let mut other = other.clone();
        other.extend(self.data.clone());
        self.data = other;
        self.last_update = Epoch::get_local();
    }
}

impl Update<HashMap<String, String>> for Cachable<HashMap<String, String>> {
    fn update(&mut self, other: &HashMap<String, String>) {
        let mut other = other.to_owned();
        other.extend(self.data.clone());
        self.data = other;
    }
}
