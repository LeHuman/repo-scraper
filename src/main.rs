mod reposcrape;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    reposcrape::test::repo::test_repo_create()?;
    reposcrape::test::store::test_bincache_encode_decode()?;
    reposcrape::query::test::github::test_github_retrieve().await?;
    Ok(())
}
