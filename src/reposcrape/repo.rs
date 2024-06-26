use bincode::{Decode, Encode};
use set_field::SetField;
use std::{cmp::Ordering, collections::HashMap};

use crate::date::EpochType;

// TODO: map details to color codes if possible, look into phf crate for static maps

#[derive(Eq, PartialEq, Encode, Decode, Default, SetField, Clone, Debug)]
pub struct RepoDetails {
    pub project: Option<String>,
    pub main: Option<String>, // NOTE: Special option that defines this repo as the main for it's project, it's value does not matter but it should have a value
    pub title: Option<String>,
    pub font: Option<Vec<String>>,
    pub color: Option<Vec<u32>>,
    pub keywords: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub technology: Option<Vec<String>>,
    pub status: Option<String>,
    pub description: Option<String>,
}

impl RepoDetails {
    fn set(&mut self, key: &String, val: &String) -> bool {
        // IMPROVE: attempt to 'cast' directly to type, instead of just trial and error, will need custom macro or just use serde
        let nkey = key.to_lowercase();
        if self.set_field(&nkey, Some(val.clone())) {
            return true;
        };

        let val_arr: Vec<String> = if val.contains(',') {
            val.split(',')
                .map(|s| s.trim()) // Trim whitespace from each substring
                .filter(|s| !s.is_empty()) // Filter out empty substrings
                .map(String::from)
                .collect()
        } else {
            vec![val.trim().to_string()]
        };
        if !val_arr.is_empty() && self.set_field(&nkey, Some(val_arr.clone())) {
            return true;
        };

        let val_ints: Vec<u32> = val_arr
            .into_iter()
            .filter_map(|s| match s.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => {
                    // NOTE: Special Hex value case for Vec<u32>
                    u32::from_str_radix(if s.starts_with('#') { &s[1..] } else { &s }, 16).ok()
                }
            })
            .collect();
        if !val_ints.is_empty() && self.set_field(&nkey, Some(val_ints.clone())) {
            return true;
        };

        return false;
    }
}

#[derive(Eq, Encode, Decode, Default, Clone, Debug)]
pub struct Repo {
    pub uid: String,
    pub id: String,
    pub name: String,
    pub owner: String,
    pub origin: String,
    pub last_sync: EpochType,
    pub last_update: EpochType,
    pub details: Option<RepoDetails>,
}

// TODO: Ensure comparing date strings works
impl Ord for Repo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return self.last_sync.cmp(&other.last_sync);
        } else {
            self.last_update
                .cmp(&other.last_update)
                .then(self.uid.cmp(&other.uid))
        }
    }
}

impl PartialOrd for Repo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Repo {
    pub fn new(
        id: String,
        name: String,
        owner: String,
        origin: String,
        last_sync: EpochType,
        last_update: EpochType,
        metadata: &HashMap<String, String>,
    ) -> Repo {
        let mut details = RepoDetails::default();
        let mut update: bool = false;
        for (key, val) in metadata {
            if !details.set(&key, &val) {
                println!("Failed to set {} {}", key, val);
            } else {
                update = true;
            }
        }
        let mut uid = origin.to_owned();
        uid.push('/');
        uid.push_str(&id.as_str());
        Repo {
            uid: uid,
            id: id,
            name: name,
            owner: owner,
            origin: origin,
            last_sync: last_sync,
            last_update: last_update,
            details: if update { Some(details) } else { None },
        }
    }
}
