extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate ts_easy_reviewer;

use docopt::Docopt;
use ts_easy_reviewer::config::get_config;
use ts_easy_reviewer::reviewer::review_files;
use ts_easy_reviewer::rule::get_rules;

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
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let config = get_config(Some(args.arg_path)).unwrap();
    println!("args.flag_version: {:?}", args.flag_version);

    let rules = get_rules(config);
    review_files(rules)
}
