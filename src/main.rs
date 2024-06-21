use std::env;

use octocrab::Octocrab;
use reposcrape::{
    cache::Cache,
    expand::ExpandedCache,
    query::{github::GHQuery, query::QueryInterface},
};

mod reposcrape;

async fn example(cache_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = Cache::load(cache_file);

    let octocrab = Octocrab::builder()
        .user_access_token(env::var("GITHUB_TOKEN").expect("No Github token"))
        .build()?;
    let query = GHQuery::new(octocrab);

    // let mut fetched = query
    // .fetch_after("LeHuman", 8, Date::from_date_str(&cache.last_update)?)
    // .await?;
    let fetched = query.fetch_latest("LeHuman", 8).await?;

    cache.update(&fetched);
    cache.save(cache_file)?;

    let expanded = ExpandedCache::new(&cache);

    println!("{:#?}", expanded);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    reposcrape::test::repo::test_repo_create()?;
    reposcrape::test::cache::test_cache_encode_decode()?;
    reposcrape::test::expand::test_expand_cache()?;
    // reposcrape::query::test::github::test_github_retrieve().await?;
    example("./.cache").await?;
    Ok(())
}
