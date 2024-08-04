use std::env;

use localsavefile::LocalSaveFile;
use octocrab::Octocrab;

use crate::{
    color,
    reposcrape::{
        cache::{ExpandedRepoCache, RepoScrapeCache, Update},
        query::{GHQuery, QueryInterface},
    },
};

#[test]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = RepoScrapeCache::load_default();

    if cache.is_empty() || cache.repos.is_outdated() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let colors = rt.block_on(color::fetch_language_colors());

        let octocrab = Octocrab::builder()
            .user_access_token(env::var("GITHUB_TOKEN").expect("No Github token"))
            .build()?;
        let query = GHQuery::new(octocrab);

        let fetched = rt.block_on(query.fetch_latest("LeHuman", 64))?;

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
