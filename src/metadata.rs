use crate::util;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub checksum: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Collection {
    pub metadata: std::collections::BTreeMap<String, Metadata>,
}

pub fn get(name: PathBuf) -> Result<Collection, util::Error> {
    let output = util::to_string(name)?;
    match serde_json::from_str(&output) {
        Ok(v) => Ok(v),
        Err(_) => Err(util::Error::ParseFailed),
    }
}

impl Collection {
    pub fn put(self) -> Result<String, util::Error> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(_) => Err(util::Error::DumpFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let c: Collection = serde_json::from_str(
            r#"
        {
            "metadata": {
                "simple": {
                    "title": "hello, world",
                    "checksum": "42"
                }
            }
        }
        "#,
        )
        .unwrap();
        assert_eq!(
            c.metadata.get("simple"),
            Some(&Metadata {
                title: String::from("hello, world"),
                checksum: String::from("42")
            })
        );
    }
}
