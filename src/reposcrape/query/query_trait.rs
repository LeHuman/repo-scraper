use crate::{date::EpochType, reposcrape::Repo};
use std::collections::BTreeSet;

pub type QueryResult = Result<BTreeSet<Repo>, Box<dyn std::error::Error>>;
pub type QueryResultSingle = Result<Repo, Box<dyn std::error::Error>>;

//TODO: pass cache to compare and reduce redundant readme data from being queried
pub trait QueryInterface {
    fn fetch_latest(
        &self,
        user: &str,
        max_count: u32,
    ) -> impl std::future::Future<Output = QueryResult>;
    fn fetch_after(
        &self,
        user: &str,
        max_count: u32,
        after_epoch: EpochType,
    ) -> impl std::future::Future<Output = QueryResult>;
    fn fetch_single(&self, url: &str) -> impl std::future::Future<Output = QueryResultSingle>;
}
