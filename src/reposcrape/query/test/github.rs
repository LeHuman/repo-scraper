use dotenv::dotenv;
use octocrab::Octocrab;
use secrecy::SecretString;

use crate::reposcrape::{
    query::{github::GHQuery, query::QueryInterface},
    Date,
};

pub async fn test_github_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let personal_token = SecretString::new(String::from(std::env::var("PERSONAL_GITHUB_TOKEN")?));
    let octocrab = Octocrab::builder().personal_token(personal_token).build()?;
    let query = GHQuery::new(octocrab);

    let _latest = query.fetch_latest("LeHuman", 4).await?;
    let _dated = query
        .fetch_after(
            "LeHuman",
            4,
            Date::from_date_str("2021-05-15")?,
        )
        .await?;

    Ok(())
}
