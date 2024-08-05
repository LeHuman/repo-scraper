use std::{env, time::Duration};

use localsavefile::LocalSaveFile;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{
    color,
    reposcrape::{
        cache::{ExpandedRepoCache, RepoScrapeCache, Update},
        query::{GHQuery, QueryInterface},
    },
};

async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = RepoScrapeCache::load_default();

    if cache.is_empty() || cache.repos.is_outdated() {
        let colors = color::fetch_language_colors().await;
        let query = GHQuery::from_personal_token(
            env::var("secrets.GITHUB_TOKEN").expect("No Github token"),
        );

        let fetched = query.fetch_latest("LeHuman", 64).await?;

        cache.repos.update(&fetched);
        if let Ok(colors) = colors {
            // TODO: Should colors be obtained through query? Or keep this as a general resource?
            cache.colors.update(&colors);
        }
        cache.save()?;
    }

    let expanded = ExpandedRepoCache::new(cache);

    println!("{:#?}", expanded);
    Ok(())
}

#[test]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    dotenvy::dotenv()?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let a = rt.block_on(run_example());
    rt.shutdown_timeout(Duration::from_secs(60));
    a
}
