use regex::Regex;
use config;

#[derive(Debug)]
pub struct RuleType {
    rule_type: Option<String>,
    context: Option<String>,
}

impl RuleType {
    pub fn new() -> RuleType {
        RuleType {rule_type: None, context: None}
    }

    pub fn set_rule_type(&mut self, v: &str) {
        match v {
            "class_implements" => {
                self.rule_type = Some("class_implements".to_string());
            },
            "class_extends" => {
                self.rule_type = Some("class_extends".to_string());
            },
            "method_returns" => {
                self.rule_type = Some("method_returns".to_string());
            },
            _ => {}
        }
    }

    pub fn set_context(&mut self, v: &str) {
        self.context = Some(v.to_string());
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    statement: String,
    rule_type: RuleType,
}

impl Rule {
    pub fn new(name: String, statement: String) -> Rule {
        let cloned_statement = statement.clone();
        let tokens = cloned_statement.split(" ");
        let rule_type = tokens.enumerate().fold(RuleType::new(), |mut acc: RuleType, (i, v)| {
            match i {
                0 => {
                    acc.set_rule_type(v);
                    acc
                },
                1 => {
                    acc.set_context(v);
                    acc
                },
                _ => acc
            }
        });
        Rule {name, statement, rule_type}
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
