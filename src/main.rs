extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs::{self, DirEntry, File};
use std::path::{Path, PathBuf};
use docopt::Docopt;
use toml::Value as Toml;

const USAGE: &'static str = "
Usage:
  tser
  tser <path>
  tser (-h | --help)
  tser (-v | --version)

Options:
  -c --config   The location of the configuration file.
  -h --help     Show this screen.
  -v --version  Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    flag_version: bool,
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn read_file(path: &Path) -> String {
    let mut buf_file = BufReader::new(File::open(path).unwrap());

    let mut buffer = String::new();
    loop {
        match buf_file.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => continue,
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }

    buffer
}

fn read_config_file() -> Toml {
    let buf_pwd = env::current_dir().unwrap();
    let pwd = buf_pwd.to_str().unwrap();
    let config_filename = "/config.toml";

    let path_base = [pwd, config_filename].join("");
    let config_file_path = Path::new(&path_base);
    let result = read_file(&config_file_path);

    toml::from_str(&result).unwrap()
}

fn get_path_string(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

fn scan_files() {
    visit_dirs(Path::new("./"), &|entry: &DirEntry| {
        let buf_path = entry.path();
        let path_string = get_path_string(&buf_path);

        if path_string.contains(".DS_Store") {
            return;
        } else if !path_string.contains(".ts") {
            return;
        }

        let result = read_file(&buf_path);

        println!("{}", result);
    }).unwrap();
}

fn main() {
    let config = read_config_file();

    println!("{:?}", config);

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("args.arg_path: {:?}", args.arg_path);
    println!("args.flag_version: {:?}", args.flag_version);

    scan_files()
}
