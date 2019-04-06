//! Misc utility functions.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Error commonly used across this crate.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Not run on a supported OS.
    NotSupported,
    /// Didn't find the given resource.
    NotFound,
    /// Failed to parse some input.
    ParseFailed,
    /// Failed to dump some object.
    DumpFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

/// Get the xdg dir usually saved in `$var`. defaults to `$HOME/path_from_home`.
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

/// Try loading the content at the given `filepath` into a `String`.
pub fn to_string(filepath: PathBuf) -> Result<String, Error> {
    match fs::read_to_string(filepath) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::NotFound),
    }
}

/// Try finding the config file. Use the one passed if it's `Some`.
pub fn get_config_file(config_dir_path: Option<String>) -> Result<PathBuf, Error> {
    let config_file = match config_dir_path {
        None => get_xdg_dir("XDG_CONFIG_HOME", ".config/")?.join("rsst/config.toml"),
        Some(fp) => PathBuf::from(fp),
    };
    Ok(config_file)
}

/// Try finding the metadata dir. Use the one passed if it's `Some`.
pub fn get_metadata_dir(meta_dir_path: Option<String>) -> Result<PathBuf, Error> {
    let metadata_dir = match meta_dir_path {
        None => get_xdg_dir("XDG_DATA_HOME", ".local/share")?,
        Some(fp) => PathBuf::from(fp),
    }
    .join("rsst");
    Ok(metadata_dir)
}

/// Try finding the output dir. Use the one passed if it's `Some`.
pub fn get_output_dir(output_dir_path: Option<String>) -> Result<PathBuf, Error> {
    match output_dir_path {
        None => get_xdg_dir("RSST_FOLDER", "rsst"),
        Some(fp) => Ok(PathBuf::from(fp)),
    }
}
