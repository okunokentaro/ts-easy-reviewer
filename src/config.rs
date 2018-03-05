extern crate toml;
use std::{env, path, result};
use reader::read_file;

type Result<T> = result::Result<T, String>;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: Option<String>,
    pub rule: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

fn read_config(mut dir: path::PathBuf) -> Result<Config> {
    let config_file_path = dir.join("config.toml");
    match read_file(&config_file_path) {
        Ok(val) => toml::from_str(&val).map_err(|e| e.to_string()),
        Err(_) => {
            dir.pop();
            read_config(dir)
        }
    }
}

pub fn get_config(path_string: &Option<String>) -> Result<Config> {
    match path_string {
        &Some(ref path_string) => {
            let path = path::PathBuf::from(path_string);
            read_config(path)
        }
        &None => env::current_dir()
            .map_err(|e| e.to_string())
            .and_then(|pwd| read_config(pwd)),
    }
}
