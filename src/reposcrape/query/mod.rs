mod query_trait;
pub use query_trait::QueryInterface;
pub use query_trait::QueryResult;
pub use query_trait::QueryResultSingle;

mod github;
pub use github::GHQuery;

#[cfg(test)]
pub mod test;
