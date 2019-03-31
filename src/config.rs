use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupported,
    NotFound,
    ParseFailed,
}

#[derive(Deserialize)]
pub struct Setting {
    pub output_format: Option<String>,
    pub output_dir: Option<String>,
    pub metadata_dir: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub setting: Setting,
    pub source: std::collections::BTreeMap<String, String>,
}

fn to_string(name: Option<&str>) -> Result<String, Error> {
    let filepath = match name {
        None => {
            let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
                Ok(path) => PathBuf::from(path),
                Err(_) => PathBuf::from(match env::var("HOME") {
                    Ok(v) => v,
                    Err(_) => return Err(Error::NotSupported),
                })
                .join(".config"),
            };
            xdg_config_home.join("rsst").join("config.toml")
        }
        Some(name) => PathBuf::from(name),
    };
    match fs::read_to_string(filepath) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::NotFound),
    }
}

pub fn get(name: Option<&str>) -> Result<Config, Error> {
    match to_string(name) {
        Ok(output) => match toml::from_str(&output) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ParseFailed),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let config: Config = toml::from_str(
            r#"
            [setting]
            metadata_dir = "$HOME/.local/share/rsst"
            output_format = "markdown"
            output_dir = "$HOME/rsst/"
            [source]
            example1 = "https://example.com/rss.xml"
            example2 = "https://example.org/rss.xml"
        "#,
        )
        .unwrap();
        assert_eq!(config.setting.output_format, Some(String::from("markdown")));
        assert_eq!(config.setting.output_dir, Some(String::from("$HOME/rsst/")));
        assert_eq!(
            config.setting.metadata_dir,
            Some(String::from("$HOME/.local/share/rsst"))
        );
        assert_eq!(config.source.len(), 2);
        assert_eq!(
            config.source.get("example1"),
            Some(&"https://example.com/rss.xml".to_string())
        );
        assert_eq!(
            config.source.get("example2"),
            Some(&"https://example.org/rss.xml".to_string())
        );
    }

    #[test]
    fn parse_example_alternative_format() {
        let config: Config = toml::from_str(
            r#"
            setting.output_format = "markdown"
            source.example3 = "https://example.com/rss.xml"
            source.example4 = "https://example.org/rss.xml"
        "#,
        )
        .unwrap();
        assert_eq!(config.setting.output_format, Some(String::from("markdown")));
        assert_eq!(config.source.len(), 2);
        assert_eq!(
            config.source.get("example3"),
            Some(&"https://example.com/rss.xml".to_string())
        );
        assert_eq!(
            config.source.get("example4"),
            Some(&"https://example.org/rss.xml".to_string())
        );
    }

    #[test]
    fn to_string_none() {
        assert_eq!(super::to_string(Some("NOT_EXISTS")), Err(Error::NotFound));
    }

    #[test]
    fn to_string_xdg_dir_empty_file() {
        let fixtures = env::current_dir()
            .expect("failed to get current dir")
            .join("fixtures/empty/");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string(None), Ok(String::from("")));
    }

    #[test]
    fn to_string_xdg_dir_not_exists() {
        let fixtures = env::current_dir()
            .expect("failed to get current dir")
            .join("fixtures/NON_EXISTS/");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string(None), Err(Error::NotFound));
    }

    #[test]
    fn to_string_xdg_dir_file_not_exists() {
        let fixtures = env::current_dir()
            .expect("failed to get current dir")
            .join("fixtures/nothing/");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string(None), Err(Error::NotFound));
    }

    #[test]
    fn get_xdg_dir_simple_example() {
        let fixtures = env::current_dir()
            .expect("failed to get current dir")
            .join("fixtures/simple/");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        let config = get(None).unwrap();
        assert_eq!(config.setting.output_format, Some(String::from("html")));
        assert_eq!(config.source.len(), 2);
        assert_eq!(
            config.source.get("mine"),
            Some(&"https://quinoa42.github.io/rss.xml".to_string())
        );
        assert_eq!(
            config.source.get("again"),
            Some(&"quinoa42.github.io/rss.xml".to_string())
        );
    }

    #[test]
    fn get_simple_example() {
        let filepath = env::current_dir()
            .expect("failed to get current dir")
            .join("fixtures/simple/rsst/config.toml");
        let config = get(filepath.to_str()).unwrap();
        assert_eq!(config.setting.output_format, Some(String::from("html")));
        assert_eq!(config.source.len(), 2);
        assert_eq!(
            config.source.get("mine"),
            Some(&"https://quinoa42.github.io/rss.xml".to_string())
        );
        assert_eq!(
            config.source.get("again"),
            Some(&"quinoa42.github.io/rss.xml".to_string())
        );
    }
}
