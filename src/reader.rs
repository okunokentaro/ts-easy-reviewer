extern crate toml;

use std::{fs, path, io};
use std::io::BufRead;
use config::Config;

pub fn read_file(path: &path::Path) -> io::Result<String> {
    fs::File::open(path).and_then(|file| {
        let mut buf_file = io::BufReader::new(file);

        let mut buffer = String::new();
        loop {
            match buf_file.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(buffer)
    })
}

pub fn read_config(mut dir: path::PathBuf) -> Result<Config, String> {
    let config_file_path = dir.join("config.toml");
    match read_file(&config_file_path) {
        Ok(val) => toml::from_str(&val).map_err(|e| e.to_string()),
        Err(_) => {
            dir.pop();
            read_config(dir)
        }
    }
}