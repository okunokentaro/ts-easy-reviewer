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

    pub fn check(&self, code: &str) {
        if self.tokens[0] == "class_implements" {
            let implements = &self.tokens[1];
            let context_re = Regex::new(&["implements ", implements].join("")).unwrap();
            let includes_implements_context = context_re.is_match(code);
            if includes_implements_context && self.tokens[2] == "and" {
                match self.tokens[3].as_ref() {
                    "includes" => {
                        let target = &self.tokens[4];
                        let target_re = Regex::new(target).unwrap();
                        let includes_target = target_re.is_match(code);
                        if includes_target && self.tokens[6] == "error" {
                            println!("REJECT!!!!!");
                        }
                    },
                    _ => ()
                }
            }
        }
        if self.tokens[0] == "class_extends" {
            let extends = &self.tokens[1];
            let context_re = Regex::new(&["extends", extends].join(" ")).unwrap();
            let includes_extends_context = context_re.is_match(code);
            if includes_extends_context && self.tokens[2] == "and" {
                match self.tokens[3].as_ref() {
                    "import" => {
                        let target = &self.tokens[4];
                        let target_re = Regex::new(&["^import.*", target].join("")).unwrap();
                        let includes_target = target_re.is_match(code);
                        if includes_target && self.tokens[6] == "error" {
                            println!("REJECT!!!!!");
                        }
                    },
                    _ => ()
                }
            }
        }
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
