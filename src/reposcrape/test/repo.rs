use std::collections::HashMap;

use crate::{date::Epoch, reposcrape::Repo};

#[test]
pub fn test_repo_create() -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata: HashMap<String, String> = HashMap::new();

    let r0 = Repo::new(
        "test".into(),
        "".into(),
        "bru".into(),
        "oof".into(),
        "okay".into(),
        Epoch::from_rfc3339("2019-06-14T08:19:26Z")?,
        Epoch::from_rfc3339("2019-06-14T08:19:26Z")?,
        &metadata,
    );

    assert!(r0.details == None);

    metadata.insert("project".into(), "pablo".into());
    metadata.insert("TItle".into(), "hello".into());
    metadata.insert(
        "color".into(),
        "#05c3a8,#AAFFa8, potato, AAc4, 256, 100000000,#100000000".into(),
    );
    metadata.insert("keyWords".into(), "".into());
    metadata.insert("FONT".into(), "IBM, Arial,".into());
    metadata.insert("invalid".into(), "huh".into());

    let r1 = Repo::new(
        "".into(),
        "".into(),
        "".into(),
        "".into(),
        "".into(),
        Epoch::from_rfc3339("2021-06-14T08:19:26Z")?,
        Epoch::from_rfc3339("2021-06-14T08:19:26Z")?,
        &metadata,
    );

    assert!(&r1.details != &None);
    let details = &r1.details.unwrap();
    assert!(details.project == Some("pablo".into()));
    assert!(details.title == Some("hello".into()));
    assert!(details.color.clone().unwrap().len() == 5);

    metadata.insert("keyWords".into(), "huh, keyor,bruh".into());
    metadata.insert("color".into(), "45".into());
    metadata.insert("languages".into(), "reust".into());
    metadata.insert("status".into(), "oops!, all, array, 😂".into());
    metadata.insert("description".into(), "".into());

    let r2 = Repo::new(
        "fff".into(),
        "".into(),
        "35".into(),
        "atw".into(),
        "gadd".into(),
        Epoch::from_rfc3339("2020-06-14T08:19:26Z")?,
        Epoch::from_rfc3339("2020-06-14T08:19:26Z")?,
        &metadata,
    );

    let details = &r2.details.unwrap();
    assert!(details.project == Some("pablo".into()));
    assert!(details.title == Some("hello".into()));
    assert!(details.keywords.clone().unwrap().len() == 3);
    assert!(details.color.clone().unwrap().len() == 1);
    assert!(details.languages.clone().unwrap() == vec!["reust"]);
    assert!(details.status == Some("oops!, all, array, 😂".into()));
    assert!(details.description.clone().unwrap().is_empty());

    Ok(())
}
