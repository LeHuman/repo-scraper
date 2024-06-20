use crate::reposcrape::repo::Repo;
use chrono::{DateTime, Utc};
use std::collections::BTreeSet;

pub type QueryResult = Result<BTreeSet<Repo>, Box<dyn std::error::Error>>;

pub trait QueryInterface {
    async fn fetch_latest(&self, user: &str, max_count: u32) -> QueryResult;
    async fn fetch_after(
        &self,
        user: &str,
        max_count: u32,
        after_date: DateTime<Utc>,
    ) -> QueryResult;
}
