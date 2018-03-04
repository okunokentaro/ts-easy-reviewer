extern crate docopt;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::io::{self, Read};
use std::fs::{self, DirEntry, File};
use std::path::Path;
use docopt::Docopt;

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

fn read_toml_file() -> String {
    let pwd_buf = env::current_dir().unwrap();
    let pwd = pwd_buf.to_str().unwrap().to_string();
    let config_filename = String::from("/config.toml");

    let path_base = [pwd, config_filename].join("");
    let config_file_path = Path::new(&path_base);

    let mut result = String::new();
    File::open(&config_file_path).and_then(|mut f| {
        f.read_to_string(&mut result)
    }).unwrap();

    result
}

fn main() {
    let toml = read_toml_file();

    println!("{:?}", toml);

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("args.arg_path: {:?}", args.arg_path);
    println!("args.flag_version: {:?}", args.flag_version);

    visit_dirs(Path::new("./"), &|v| {
        println!("L34 {:?}", v);
    }).unwrap();
}
