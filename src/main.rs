#[macro_use]
extern crate serde_derive;
extern crate ts_easy_reviewer;
extern crate docopt;
extern crate toml;

use docopt::Docopt;
use ts_easy_reviewer::config::get_config;
use ts_easy_reviewer::reviewer::review_files;

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
    let config = get_config().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("args.arg_path: {:?}", args.arg_path);
    println!("args.flag_version: {:?}", args.flag_version);

    review_files(config)
}
