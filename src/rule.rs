use config;

#[derive(Debug)]
pub struct Rule {
    name: String,
    statement: String,
}

pub fn get_rules(config: config::Config) -> Vec<Rule> {
    let toml_rules = config.rules.unwrap();

    toml_rules.into_iter().map(|toml_rule| {
        Rule {
            name: toml_rule.name.unwrap(),
            statement: toml_rule.rule.unwrap(),
        }
    }).collect()
}
