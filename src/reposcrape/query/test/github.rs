use octocrab::Octocrab;
use std::env;
use tracing::{debug, warn};

use crate::{
    date::Epoch,
    reposcrape::query::{github::GHQuery, query_trait::QueryInterface},
};

#[test]
#[tracing_test::traced_test]
pub fn test_github_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(token) = env::var("GITHUB_TOKEN") else {
        warn!("No GITHUB_TOKEN set in env. Skipping github retrieve test");
        return Ok(());
    };
    // let personal_token = SecretString::new(String::from(std::env::var("PERSONAL_GITHUB_TOKEN")?));
    let octocrab = Octocrab::builder().user_access_token(token).build()?;
    let query = GHQuery::new(octocrab);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let _latest = rt.block_on(query.fetch_latest("LeHuman", 8))?;
    let _dated =
        rt.block_on(query.fetch_after("LeHuman", 4, Epoch::from_rfc3339("2022-05-14T19:19:26Z")?))?;

    debug!("{:?}\n{:?}\n", _latest, _dated);

    Ok(())
}
