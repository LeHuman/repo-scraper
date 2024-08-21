use octocrab::Octocrab;
use std::env;
use tracing::{debug, warn};

use crate::{
    date::Epoch,
    reposcrape::query::{github::GHQuery, query_trait::QueryInterface},
};

async fn _test_github_retrieve(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let octocrab = Octocrab::builder().user_access_token(token).build()?;
    let query = GHQuery::new(octocrab);

    let _latest = query.fetch_latest("LeHuman", 8).await?;
    let _dated = query
        .fetch_after("LeHuman", 4, Epoch::from_rfc3339("2022-05-14T19:19:26Z")?)
        .await?;

    debug!("{:?}\n{:?}\n", _latest, _dated);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
pub fn test_github_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(token) = env::var("GITHUB_TOKEN") else {
        warn!("No GITHUB_TOKEN set in env. Skipping github retrieve test");
        return Ok(());
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(_test_github_retrieve(token))
}
