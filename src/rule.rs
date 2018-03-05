use regex::Regex;
use config;

#[derive(Debug)]
pub struct Rule {
    name: String,
    statement: String,
}

impl Rule {
    pub fn check(&self, code: &String) {
        println!("=================================================================");
        println!("check {}", code);
        println!("statement {}", self.statement);

        let is_class_implements = self.statement.starts_with("class_implements");
        if is_class_implements {
            let re = Regex::new(r"class_implements\s'(.*?)'").unwrap();
            let cap = re.captures(&self.statement).unwrap();
            let implements = &cap[1];
            println!("{}", implements);

            let implements_re = Regex::new(&["implements", implements].join(" ")).unwrap();
            let result = implements_re.is_match(code);
            println!("{}", result);

            let and_re = Regex::new(r"\sand\s").unwrap();
            let and_result = and_re.is_match(&self.statement);
            println!("{}", and_result);

            let includes_re = Regex::new(r"includes\s'(.*?)'").unwrap();
            let includes_re_cap = includes_re.captures(&self.statement).unwrap();
            let includes = &includes_re_cap[1];
            println!("{}", includes);

            let includes_re_2 = Regex::new(includes).unwrap();
            let result_2 = includes_re_2.is_match(code);
            println!("{}", result_2);

            if result_2 {
                let then_error_re = Regex::new(r"\sthen\serror").unwrap();
                let then_error_result = then_error_re.is_match(&self.statement);
                println!("then_error_result {}", then_error_result);
                if then_error_result {
                    println!("error!!!!");
                }
            }
        }
    }
}

pub fn get_rules(config: config::Config) -> Vec<Rule> {
    let toml_rules = config.rules.unwrap();

    toml_rules
        .into_iter()
        .map(|toml_rule| Rule {
            name: toml_rule.name.unwrap(),
            statement: toml_rule.rule.unwrap(),
        })
        .collect()
}
