use crate::reposcrape::{date::EpochType, repo::Repo};
use std::collections::BTreeSet;

pub type QueryResult = Result<BTreeSet<Repo>, Box<dyn std::error::Error>>;

//TODO: pass cache to compare and reduce redundant readme data from being queried
pub trait QueryInterface {
    async fn fetch_latest(&self, user: &str, max_count: u32) -> QueryResult;
    async fn fetch_after(&self, user: &str, max_count: u32, after_epoch: EpochType) -> QueryResult;
}
