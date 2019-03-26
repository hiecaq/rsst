use toml;
use std::env;
use std::fs;
use std::path::PathBuf;


pub fn to_string(name: &str) -> Option<String> {
    let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => PathBuf::from(env::var("HOME").expect("OS not supported")).join(".config"),
    };
    match fs::read_to_string(xdg_config_home.join(name).join("config.toml")) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_none() {
        assert_eq!(super::to_string("NOT_EXISTS"), Option::None);
    }

    #[test]
    fn to_string_declared_dir_empty_file() {
        let fixtures = env::current_dir().expect("failed to get current dir").join("fixtures");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string("empty"), Option::Some(String::from("")));
    }

    #[test]
    fn to_string_declared_dir_not_exists() {
        let fixtures = env::current_dir().expect("failed to get current dir").join("fixtures");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string("NOT_EXISTS"), None);
    }

    #[test]
    fn to_string_declared_dir_file_not_exists() {
        let fixtures = env::current_dir().expect("failed to get current dir").join("fixtures");
        env::set_var("XDG_CONFIG_HOME", fixtures);
        assert_eq!(to_string("nothing"), None);
    }
}
