#[derive(Debug)]
struct Parser {
    cursor: Cursor,
    tokens: Vec<String>,
}

impl Parser {
    pub fn new(cursor: Cursor) -> Parser {
        Parser {
            cursor,
            tokens: Vec::new(),
        }
    }
    pub fn update_cursor(mut self, mode: String, n: usize) -> Parser {
        self.cursor = self.cursor.update(mode, n);
        self
    }
    pub fn add_token(mut self, token: String) -> Parser {
        self.tokens.push(token);
        self
    }
}

#[derive(Debug)]
struct Cursor {
    mode: String,
    n: usize,
}

impl Cursor {
    pub fn update(mut self, mode: String, n: usize) -> Cursor {
        self.mode = mode;
        self.n = n;
        self
    }
}

fn get_string(vec_chars: &Vec<char>, begin: usize, end: usize) -> String {
    vec_chars[begin..end].iter().cloned().collect::<String>()
}

pub fn parse(statement: &str) -> Vec<String> {
    let vec_chars = statement.chars().collect::<Vec<char>>();

    let default_cursor = Cursor {
        mode: "statement".to_string(),
        n: 0,
    };
    let parser = Parser::new(default_cursor);
    let result = vec_chars
        .clone()
        .into_iter()
        .enumerate()
        .fold(parser, |mut parser, (i, v)| {
            match parser.cursor.mode.as_ref() {
                "statement" => {
                    match v {
                        'c' => {
                            // class_implements, class_extends
                            parser.update_cursor("indicator".to_string(), i)
                        }
                        'm' => {
                            // method_returns
                            parser.update_cursor("indicator".to_string(), i)
                        }
                        _ => {
                            println!("error 100");
                            parser
                        }
                    }
                }
                "indicator" => match v {
                    ' ' => {
                        let token = get_string(&vec_chars, parser.cursor.n, i);
                        parser = parser.add_token(token);
                        parser.update_cursor("start_identifier".to_string(), i)
                    }
                    _ => parser,
                },
                "start_identifier" => match v {
                    '\'' => parser.update_cursor("identifier".to_string(), i + 1),
                    _ => parser,
                },
                "identifier" => match v {
                    '\'' => {
                        let token = get_string(&vec_chars, parser.cursor.n, i);
                        parser = parser.add_token(token);
                        parser.update_cursor("end_identifier".to_string(), i + 1)
                    }
                    _ => parser,
                },
                "end_identifier" => match v {
                    ' ' => parser.update_cursor("and_then_statement".to_string(), i + 1),
                    _ => parser,
                },
                "and_then_statement" => {
                    match v {
                        // and
                        'a' => parser.update_cursor("and_indicator".to_string(), i),
                        // then
                        't' => parser.update_cursor("then_indicator".to_string(), i),
                        _ => parser,
                    }
                }
                "and_statement" => {
                    match v {
                        // includes, import
                        'i' => parser.update_cursor("and_indicator".to_string(), i),
                        // not, name_matches
                        'n' => {
                            if get_string(&vec_chars, parser.cursor.n, parser.cursor.n + 3) == "not"
                            {
                                return parser.update_cursor("and_indicator".to_string(), i);
                            } else if get_string(&vec_chars, parser.cursor.n, parser.cursor.n + 12)
                                == "name_matches"
                            {
                                return parser.update_cursor("and_statement".to_string(), i);
                            }
                            parser
                        }
                        '\'' => parser.update_cursor("identifier".to_string(), i + 1),
                        ' ' => {
                            let token = get_string(&vec_chars, parser.cursor.n, i);
                            parser = parser.add_token(token);
                            parser.update_cursor("and_statement".to_string(), i + 1)
                        }
                        _ => parser,
                    }
                }
                "and_indicator" => match v {
                    ' ' => {
                        let token = get_string(&vec_chars, parser.cursor.n, i);
                        parser = parser.add_token(token);
                        parser.update_cursor("and_statement".to_string(), i + 1)
                    }
                    _ => parser,
                },
                "then_statement" => {
                    match v {
                        // error
                        'e' => parser.update_cursor("then_indicator".to_string(), i),
                        // warn
                        'w' => parser.update_cursor("then_indicator".to_string(), i),
                        _ => parser,
                    }
                }
                "then_indicator" => {
                    if vec_chars.len() - 1 == i {
                        return match get_string(&vec_chars, parser.cursor.n, i + 1).as_ref() {
                            "error" => {
                                let token = get_string(&vec_chars, parser.cursor.n, i + 1);
                                parser = parser.add_token(token);
                                parser.update_cursor("statement".to_string(), i)
                            }
                            "warn" => {
                                let token = get_string(&vec_chars, parser.cursor.n, i + 1);
                                parser = parser.add_token(token);
                                parser.update_cursor("statement".to_string(), i)
                            }
                            _ => parser,
                        };
                    }
                    match v {
                        ' ' => {
                            let token = get_string(&vec_chars, parser.cursor.n, i);
                            parser = parser.add_token(token);
                            parser.update_cursor("then_statement".to_string(), i + 1)
                        }
                        _ => parser,
                    }
                }
                _ => {
                    println!("error 9000");
                    parser
                }
            }
        });

    result.tokens
}
