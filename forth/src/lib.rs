use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
    vec,
};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type Definition = Rc<Vec<Token>>;

#[derive(Debug)]
enum Token {
    Tok(String),
    Def(Definition),
}

impl Token {
    pub fn to_tokens(&self) -> Vec<String> {
        match self {
            Token::Tok(token) => vec![token.to_string()],
            Token::Def(tokens) => tokens.iter().flat_map(|t| t.to_tokens()).collect(),
        }
    }
}

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, Definition>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_lowercase();
        let mut tokens = input
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<VecDeque<_>>();
        while let Some(w) = tokens.pop_front() {
            if w == ":" {
                // We have a definition statement
                self.save_definition(&mut tokens)?;
            } else if self.definitions.contains_key(&w) {
                // We have a variable here. Remove it from the token queue and
                // put the corresponding tokens instead.
                let w = self.definitions.get(&w).unwrap();
                for t in w.iter().flat_map(|t| t.to_tokens()).rev() {
                    tokens.push_front(t);
                }
            } else {
                self.handle_tokens(w)?;
            }
        }
        Ok(())
    }

    fn handle_tokens(&mut self, w: String) -> Result {
        match w.as_str() {
            n if n.parse::<Value>().is_ok() => self.stack.push(n.parse::<Value>().unwrap()),
            "/" | "+" | "-" | "*" => {
                let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let res = match w.as_str() {
                    "/" => y.checked_div(x).ok_or(Error::DivisionByZero)?,
                    "*" => y * x,
                    "+" => y + x,
                    "-" => y - x,
                    _ => panic!("Invalid state"),
                };
                self.stack.push(res);
            }
            "drop" => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            "dup" => {
                let val = self.stack.last().ok_or(Error::StackUnderflow)?;
                self.stack.push(*val);
            }
            "swap" => {
                let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(x);
                self.stack.push(y);
            }
            "over" => {
                let len = self.stack.len();
                if len < 2 {
                    return Err(Error::StackUnderflow);
                }
                let val = self.stack.get(len - 2).unwrap();
                self.stack.push(*val);
            }
            _ => return Err(Error::UnknownWord),
        };
        Ok(())
    }

    /// Consumes the definition statement from the tokens and
    /// saves them.
    fn save_definition(&mut self, tokens: &mut VecDeque<String>) -> Result {
        let mut n_tokens = vec![];
        while let Some(t) = tokens.pop_front() {
            n_tokens.push(t.clone());
            if t == ";" {
                break;
            }
        }
        // Each definition has to end with ;
        if n_tokens.pop().ok_or(Error::InvalidWord)? != ";" {
            return Err(Error::InvalidWord);
        }
        // Definitions can not start with numbers
        let (name, n_tokens) = n_tokens.split_first().unwrap();
        if name.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord);
        }
        // Convert the words into corresponding tokens
        // If the token refers to a previous definition
        // put a reference to it.
        let v = n_tokens
            .iter()
            .map(|s| {
                if let Some(t) = self.definitions.get(&s.to_string()) {
                    Token::Def(t.clone())
                } else {
                    Token::Tok(s.clone())
                }
            })
            .collect();
        self.definitions.insert(name.to_string(), Rc::new(v));
        Ok(())
    }
}
