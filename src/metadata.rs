//! Code that manipulates the metadata file.

use crate::util;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;

/// A metadata entry for a feed.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Metadata {
    /// the title of this feed.
    pub title: String,
    /// the checksum in md5 that marks the last newest article.
    pub checksum: String,
}

/// A collection that maps alias to metadata for each feed.
#[derive(Deserialize, Serialize, Default)]
pub struct Collection {
    pub metadata: std::collections::BTreeMap<String, Metadata>,
}

/// Try deserializing the file at the given `PathBuf` into a `Collection`.
pub fn get(name: PathBuf) -> Result<Collection, util::Error> {
    let output = util::to_string(name)?;
    match serde_json::from_str(&output) {
        Ok(v) => Ok(v),
        Err(_) => Err(util::Error::ParseFailed),
    }
}

impl Collection {
    /// Try Serializing `self` into a `String`.
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
