use path::get_path_string;
use std::path;
use regex::Regex;
use config;
use statement_parser::parse;

#[derive(Debug)]
pub struct Report {
    pub rejected: Vec<String>,
}

impl Report {
    pub fn new() -> Report {
        Report { rejected: vec![] }
    }

    pub fn add_rejected(&mut self, path: &path::PathBuf, line: usize, statement: &str) {
        self.rejected.push(
            [
                get_path_string(path),
                ":".to_string(),
                line.to_string(),
                " ".to_string(),
                statement.to_string(),
            ].join(""),
        )
    }

    pub fn exists(&self) -> bool {
        0 < self.rejected.len()
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    statement: String,
    tokens: Vec<String>,
}

impl Rule {
    pub fn new(name: String, statement: String) -> Rule {
        let tokens = parse(&statement);
        Rule {
            name,
            statement,
            tokens,
        }
    }

    pub fn check(&self, file_path: &path::PathBuf, code: &str) -> Report {
        if self.tokens[0] == "class_implements" {
            let implements = &self.tokens[1];
            let context_re = Regex::new(&["implements ", implements].join("")).unwrap();
            let includes_implements_context = context_re.is_match(code);
            if includes_implements_context && self.tokens[2] == "and" {
                return match self.tokens[3].as_ref() {
                    "includes" => {
                        let target = &self.tokens[4];
                        let target_re = Regex::new(target).unwrap();
                        let includes_target = target_re.is_match(code);
                        if includes_target && self.tokens[6] == "error" {
                            let lines = code.split("\n").enumerate();
                            return lines.fold(Report::new(), |mut report, (i, line)| {
                                if target_re.is_match(line) {
                                    report.add_rejected(file_path, i + 1, &self.statement);
                                }
                                report
                            });
                        }
                        Report::new()
                    }
                    _ => Report::new(),
                };
            }
        }
        if self.tokens[0] == "class_extends" {
            let extends = &self.tokens[1];
            let context_re = Regex::new(&["extends", extends].join(" ")).unwrap();
            let includes_extends_context = context_re.is_match(code);
            if includes_extends_context && self.tokens[2] == "and" {
                return match self.tokens[3].as_ref() {
                    "import" => {
                        let target = &self.tokens[4];
                        let target_re = Regex::new(&["^import.*", target].join("")).unwrap();
                        let includes_target = target_re.is_match(code);
                        if includes_target && self.tokens[6] == "error" {
                            let lines = code.split("\n").enumerate();
                            return lines.fold(Report::new(), |mut report, (i, line)| {
                                if target_re.is_match(line) {
                                    report.add_rejected(file_path, i + 1, &self.statement);
                                }
                                report
                            });
                        }
                        Report::new()
                    }
                    _ => Report::new(),
                };
            }
        }
        Report::new()
    }
}

pub fn get_rules(config: config::Config) -> Vec<Rule> {
    let toml_rules = config.rules.unwrap();

    toml_rules
        .into_iter()
        .map(|toml_rule| Rule::new(toml_rule.name.unwrap(), toml_rule.rule.unwrap()))
        .collect()
}
