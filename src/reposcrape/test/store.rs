use std::collections::BTreeSet;

use crate::reposcrape::{
    repo::{Repo, RepoDetails},
    store::BinCache,
};

pub fn test_bincache_encode_decode() -> Result<(), Box<dyn std::error::Error>> {
    let mut repos: BTreeSet<Repo> = BTreeSet::new();
    repos.insert(Repo {
        uid: "github/Username/Repo0".into(),
        id: "Username/Repo0".into(),
        name: "Repo0".into(),
        owner: "Username0".into(),
        origin: "github".into(),
        last_update: "2018-04-06".into(),
        details: Some(RepoDetails {
            project: Some("Project0".into()),
            title: Some("Repo0-0".into()),
            font: Some(Vec::from(["Arial".into(), "Helvetica".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key0".into(), "yek1".into()])),
            description: Some("Description0".into()),
            languages: Some(Vec::from(["rust".into(), "C++".into()])),
            technology: Some(Vec::from(["GH Actions".into(), "https".into()])),
            status: Some("Work In Progress".into()),
        }),
    });
    repos.insert(Repo {
        uid: "github/Username/Repo1".into(),
        id: "Username/Repo1".into(),
        name: "Repo1".into(),
        owner: "Username1".into(),
        origin: "github".into(),
        last_update: "2020-08-07".into(),
        details: Some(RepoDetails {
            project: Some("Project0".into()),
            title: Some("Repo1-0".into()),
            font: Some(Vec::from(["Times new roman".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key2".into(), "yek3".into()])),
            description: Some("Description1".into()),
            languages: Some(Vec::from(["C++".into()])),
            technology: Some(Vec::from(["https".into()])),
            status: Some("Archive".into()),
        }),
    });
    repos.insert(Repo {
        uid: "github/Username/Repo2".into(),
        id: "Username/Repo2".into(),
        name: "Repo2".into(),
        owner: "Username0".into(),
        origin: "github".into(),
        last_update: "2020-07-02".into(),
        details: Some(RepoDetails {
            project: Some("Project1".into()),
            title: Some("Repo2-1".into()),
            font: Some(Vec::from(["Arial".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key0".into(), "yek3".into()])),
            description: Some("Description2 😎".into()),
            languages: Some(Vec::from(["C".into()])),
            technology: Some(Vec::from(["tech2".into()])),
            status: Some("Work In Progress".into()),
        }),
    });

    let dummy_load_start = BinCache::new(repos);

    let dump = dummy_load_start.dump()?;

    let dummy_load_end = BinCache::load(&dump)?;

    assert!(dummy_load_end == dummy_load_start);

    Ok(())
}
