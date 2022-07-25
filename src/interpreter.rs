use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap
};

#[derive(Debug)]
enum Tokens {
    INT(i32),
    KEYWORD(KEYWORD),
    OP(Op),
}

#[derive(Debug)]
enum KEYWORD {
    LET,
    VARNAME(String),
}

#[derive(Debug)]
enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    EQL,
}

pub struct Interpreter {
    lines: Vec<String>,
    pointer: i32,
    variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new(source: String) -> Interpreter {
        let file = File::open(source).expect("file not found");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        Interpreter {
            lines,
            pointer: 0,
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        loop {
            if self.pointer >= self.lines.len() as i32 {
                break;
            }

            let line = &self.lines[self.pointer as usize];
            let tokens = tokenize(line.to_string());
            //OP loop
            for i in 0..tokens.len() {
                if let Tokens::OP(op) = &tokens[i] {
                    
                    let result: Option<i32> = match op {
                        Op::ADD => {
                            let vars = match self.get_left_right(&tokens, i) {
                                Ok(vars) => vars,
                                Err(e) => return Err(e),
                            };
                            Some(vars.0 + vars.1)
                        },
                        Op::SUB => {
                            let vars = match self.get_left_right(&tokens, i) {
                                Ok(vars) => vars,
                                Err(e) => return Err(e),
                            };
                            Some(vars.0 - vars.1)
                        },
                        Op::MUL => {
                            let vars = match self.get_left_right(&tokens, i) {
                                Ok(vars) => vars,
                                Err(e) => return Err(e),
                            };
                            Some(vars.0 * vars.1)
                        },
                        Op::DIV => {
                            let vars = match self.get_left_right(&tokens, i) {
                                Ok(vars) => vars,
                                Err(e) => return Err(e),
                            };
                            Some(vars.0 / vars.1)
                        },
                        Op::EQL => {
                            if i < 2 {
                                return Err(format!("Missing keywords for setting variable on line {}", self.get_line()));
                            }

                            match tokens[i - 2] {
                                Tokens::KEYWORD(KEYWORD::LET) => (),
                                _ => return Err(format!("let keyword not found for setting variable on line {}", self.get_line())),
                            };

                            let varname = match &tokens[i - 1] {
                                Tokens::KEYWORD(KEYWORD::VARNAME(varname)) => varname,
                                _ => return Err(format!("variable name not found for setting variable on line {}", self.get_line())),
                            };

                            let value = match &tokens[i + 1] {
                                Tokens::INT(value) => value,
                                Tokens::KEYWORD(KEYWORD::VARNAME(value)) => {
                                    if let Some(value) = self.variables.get(value) {
                                        value
                                    } else {
                                        return Err(format!("variable {} not found for setting variable on line {}", value, self.get_line()));
                                    }
                                },
                                _ => return Err(format!("Expected intlit on the left of op for setting variable on line {}", self.get_line())),
                            };

                            self.variables.insert(varname.to_string(), *value);
                            None
                        },
                    };
                    if let Some(result) = result {
                        println!("{}", result);
                    }
                }
            }
            
            self.pointer += 1;
        }
        Ok(())
    }

    fn get_left_right(&self, tokens: &Vec<Tokens>, i: usize) -> Result<(i32, i32), String> {
        let left = match &tokens[i - 1] {
            Tokens::INT(i) => i,
            Tokens::KEYWORD(KEYWORD::VARNAME(varname)) => {
                match self.variables.get(varname) {
                    Some(i) => i,
                    None => return Err(format!("Variable {} not found on line {}", varname, self.get_line())),
                }
            },
            e => return Err(format!("Expected intlit on the left of op instead of {:?} on line {}", e, self.get_line())),
        };
        let right = match &tokens[i + 1] {
            Tokens::INT(i) => i,
            Tokens::KEYWORD(KEYWORD::VARNAME(varname)) => {
                match self.variables.get(varname) {
                    Some(i) => i,
                    None => return Err(format!("Variable {} not found on line {}", varname, self.get_line())),
                }
            },
            e => return Err(format!("Expected intlit on the right of op instead of {:?} on line {}", e, self.get_line())),
        };
        Ok((*left, *right))
    }

    fn get_line(&self) -> String {
        (self.pointer + 1).to_string()
    }

}

fn tokenize(line: String) -> Vec<Tokens> {
    let mut _indents = 0;
    for i in 0..line.len() {
        if line.chars().nth(i) == Some(' ') {
            _indents += 1;
        }
    }
    let mut tokens = Vec::new();
    //tokens.push(Tokens::INDENT(indents));

    let split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    for token in split {
        match token.as_str() {
            "let" => tokens.push(Tokens::KEYWORD(KEYWORD::LET)),
            "+" => tokens.push(Tokens::OP(Op::ADD)),
            "-" => tokens.push(Tokens::OP(Op::SUB)),
            "*" => tokens.push(Tokens::OP(Op::MUL)),
            "/" => tokens.push(Tokens::OP(Op::DIV)),
            "=" => tokens.push(Tokens::OP(Op::EQL)),
            _ => {
                //variable or intlit
                if token.chars().all(char::is_alphabetic) {
                    tokens.push(Tokens::KEYWORD(KEYWORD::VARNAME(token)));
                } else {
                    tokens.push(Tokens::INT(token.parse::<i32>().unwrap()));
                }
            }
        }
    }
    tokens
}