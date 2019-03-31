use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupported,
    NotFound,
    ParseFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

fn get_xdg_dir(var: &str, path_from_home: &str) -> Result<PathBuf, Error> {
    Ok(match env::var(var) {
        Ok(path) => PathBuf::from(path),
        Err(_) => PathBuf::from(match env::var("HOME") {
            Ok(v) => v,
            Err(_) => return Err(Error::NotSupported),
        })
        .join(path_from_home),
    })
}

pub fn to_string(filepath: Option<PathBuf>) -> Result<String, Error> {
    let filepath = match filepath {
        None => get_xdg_dir("XDG_CONFIG_HOME", ".config")?
            .join("rsst")
            .join("config.toml"),
        Some(fp) => fp,
    };
    match fs::read_to_string(filepath) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::NotFound),
    }
}

pub fn get_metadata_dir(meta_dir_path: Option<String>) -> Result<PathBuf, Error> {
    let metadata_dir = match meta_dir_path {
        None => get_xdg_dir("XDG_DATA_HOME", ".local/share")?,
        Some(fp) => PathBuf::from(fp),
    }
    .join("rsst");
    Ok(metadata_dir)
}
