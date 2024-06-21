use dotenv::dotenv;
use octocrab::Octocrab;
use secrecy::SecretString;

use crate::reposcrape::{
    query::{github::GHQuery, query::QueryInterface},
    Epoch,
};

pub async fn test_github_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let personal_token = SecretString::new(String::from(std::env::var("PERSONAL_GITHUB_TOKEN")?));
    let octocrab = Octocrab::builder().personal_token(personal_token).build()?;
    let query = GHQuery::new(octocrab);

    let _latest = query.fetch_latest("LeHuman", 8).await?;
    let _dated = query
        .fetch_after(
            "LeHuman",
            4,
            Epoch::from_rfc3339("2022-05-14T19:19:26Z")?,
        )
        .await?;

    Ok(())
}
