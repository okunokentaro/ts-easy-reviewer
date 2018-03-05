use regex::Regex;
use config;
use statement_parser::parse;

#[derive(Debug)]
pub struct Rule {
    name: String,
    tokens: Vec<String>
}

impl Rule {
    pub fn new(name: String, statement: String) -> Rule {
        let tokens = parse(&statement);
        Rule {name, tokens}
    }

    pub fn check(&self, code: &String) {

    }
}

pub fn get_rules(config: config::Config) -> Vec<Rule> {
    let toml_rules = config.rules.unwrap();

    toml_rules
        .into_iter()
        .map(|toml_rule| {
            Rule::new(toml_rule.name.unwrap(), toml_rule.rule.unwrap())
        })
        .collect()
}
