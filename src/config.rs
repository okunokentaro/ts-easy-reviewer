#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: Option<String>,
    pub includes: Option<String>,
    pub and_if_includes: Option<String>,
    pub then: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}
