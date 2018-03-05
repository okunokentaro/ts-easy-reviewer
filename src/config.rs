extern crate toml;
use std::{env, path};
use reader::read_file;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: Option<String>,
    pub implements: Option<String>,
    pub and_if_includes: Option<String>,
    pub then: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

fn read_config(mut dir: path::PathBuf) -> Result<Config, String> {
    let config_file_path = dir.join("config.toml");
    match read_file(&config_file_path) {
        Ok(val) => toml::from_str(&val).map_err(|e| e.to_string()),
        Err(_) => {
            dir.pop();
            read_config(dir)
        }
    }
}

pub fn get_config() -> Result<Config, String> {
    env::current_dir()
        .map_err(|e| e.to_string())
        .and_then(|pwd| {
            read_config(pwd)
        })
}
