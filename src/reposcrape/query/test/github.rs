use octocrab::Octocrab;
use std::env;

use crate::reposcrape::{
    query::{github::GHQuery, query::QueryInterface},
    Epoch,
};

pub async fn test_github_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    // let personal_token = SecretString::new(String::from(std::env::var("PERSONAL_GITHUB_TOKEN")?));
    let octocrab = Octocrab::builder()
        .user_access_token(env::var("GITHUB_TOKEN").expect("No Github token"))
        .build()?;
    let query = GHQuery::new(octocrab);

    let _latest = query.fetch_latest("LeHuman", 8).await?;
    let _dated = query
        .fetch_after("LeHuman", 4, Epoch::from_rfc3339("2022-05-14T19:19:26Z")?)
        .await?;

    Ok(())
}
