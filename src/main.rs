mod reposcrape;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    reposcrape::test::repo::test_repo_create()?;
    reposcrape::test::cache::test_cache_encode_decode()?;
    reposcrape::test::expand::test_expand_cache()?;
    reposcrape::query::test::github::test_github_retrieve().await?;
    Ok(())
}
