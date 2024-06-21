use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use octocrab::Octocrab;
use rand::Rng;

use reposcrape::{
    cache::Cache,
    expand::ExpandedCache,
    query::{github::GHQuery, query::QueryInterface},
};
mod reposcrape;

fn html_test() {
    // Define the directory and file paths
    let out_dir = "./public";
    let index_path = &format!("{}/index.html", out_dir);

    // Use the GitHub token
    let _github_token = env::var("GITHUB_TOKEN").expect("No GitHub token in env");

    // Create the directory if it doesn't exist
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Failed to create output directory");
    }

    // Define the content for the index.html
    let html_content = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>repo-scraper</title>
        </head>
        <body>
            <h1>Sup, I changed this text because I wanted to see if it updates correctly 😎</h1>
            <p>This is a test site generated by a Rust program.</p>
        </body>
        </html>
        "#;

    // Write the content to the index.html file
    let mut file = fs::File::create(index_path).expect("Failed to create index.html file");
    file.write_all(html_content.as_bytes())
        .expect("Failed to write to index.html file");

    println!("Static site generated successfully at {}", index_path);
}

fn cache_test() {
    let file_path = "./.cache";

    // Check if the file already exists
    if Path::new(file_path).exists() {
        println!("File already exists: {file_path}");
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(file_path)
            .expect("Failed to open .cache file");
        let mut rng = rand::thread_rng();
        let random_number = rng.gen::<u32>().to_string();
        file.write_all(random_number.as_bytes())
            .expect("Failed to write to file");
    } else {
        // Create and write to the file
        let mut file = fs::File::create(file_path).expect("Failed to create .cache file");
        file.write_all(b"Hello, this is a sample text.")
            .expect("Failed to write to file");
        println!("File written to {file_path}");
    }
}

async fn example(cache_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = Cache::load(cache_file);

    let octocrab = Octocrab::builder()
        .user_access_token(env::var("GITHUB_TOKEN").expect("No Github token"))
        .build()?;
    let query = GHQuery::new(octocrab);

    let fetched = query.fetch_latest("LeHuman", 8).await?;

    cache.update(&fetched);
    cache.save(cache_file)?;

    let expanded = ExpandedCache::new(&cache);

    println!("{:#?}", expanded);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // reposcrape::test::repo::test_repo_create()?;
    // reposcrape::test::cache::test_cache_encode_decode()?;
    // reposcrape::test::expand::test_expand_cache()?;
    // reposcrape::query::test::github::test_github_retrieve().await?;
    example("./.cache").await?;
    html_test();
    Ok(())
}
