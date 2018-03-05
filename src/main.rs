#[macro_use]
extern crate serde_derive;
extern crate ts_easy_reviewer;
extern crate docopt;
extern crate toml;

use std::{env, fs, io, path};
use docopt::Docopt;
use ts_easy_reviewer::reader::{read_file, read_config};

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

fn visit_dirs(dir: &path::Path, cb: &Fn(&fs::DirEntry)) -> io::Result<()> {
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

fn get_path_string(path: &path::PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

fn scan_files() {
    visit_dirs(path::Path::new("./"), &|entry: &fs::DirEntry| {
        let buf_path = entry.path();
        let path_string = get_path_string(&buf_path);

        if path_string.contains(".DS_Store") {
            return;
        } else if !path_string.contains(".ts") {
            return;
        }

        let result = read_file(&buf_path).unwrap();

        println!("{:?}", &buf_path);
        println!("{}", result);
    }).unwrap();
}

fn main() {
    let config = env::current_dir()
        .map_err(|e| e.to_string())
        .and_then(|pwd| {
            read_config(pwd)
        });

    println!("{:?}", config.unwrap().rules.unwrap()[0]);

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("args.arg_path: {:?}", args.arg_path);
    println!("args.flag_version: {:?}", args.flag_version);

    scan_files()
}
