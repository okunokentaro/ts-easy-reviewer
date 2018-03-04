#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs;
use docopt::Docopt;

const USAGE: &'static str = "
Usage:
  tser <path>
  tser (-h | --help)
  tser (-v | --version)

Options:
  -h --help     Show this screen.
  -v --version  Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("args.arg_path: {:?}", args.arg_path);

    let paths = fs::read_dir("./fixture").unwrap();

    for path in paths {
        let path_str = path.unwrap().path().display().to_string();
        println!("Name: {}", path_str);
        println!("Name: {}", args.arg_path == path_str);
    }
}
