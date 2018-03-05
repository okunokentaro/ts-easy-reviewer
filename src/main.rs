#[macro_use]
extern crate serde_derive;
extern crate ts_easy_reviewer;
extern crate docopt;
extern crate toml;

use std::env;
use docopt::Docopt;
use ts_easy_reviewer::reader::read_config;
use ts_easy_reviewer::linter::lint_files;

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

    lint_files()
}
