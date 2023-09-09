use std::process;

use crate::query::query;

#[derive(Default, Debug, PartialEq)]
pub enum TokenKind {
    #[default]
    Exit,

    IntLit,
    Semi,
}

#[derive(Default, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

pub struct Lexer {
    source: Vec<char>,
    index: i32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let source = source.chars().collect();

        Self { source, index: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buf: Vec<char> = Vec::new();

        while self.peek(1).is_some() {
            if self.peek(1).unwrap().is_alphabetic() {
                buf.push(self.consume());

                while self.peek(1).unwrap().is_alphanumeric() {
                    buf.push(self.consume());
                }

                let accumulated: String = buf.iter().map(|a| a.to_string()).collect();

                log::info!("Accumulated: \"{accumulated}\"");

                if &accumulated == "return" {
                    // Exit token
                    tokens.push(Token::default());

                    buf.clear();
                    continue;
                } else {
                    log::warn!("Accumulated: \"{accumulated}\"");
                    log::error!("Temporary error. Something went wrong.");

                    if query("Do you want to display a debug of the buffer?") {
                        dbg!(&buf);
                    }

                    process::exit(-1);
                }
            } else if self.peek(1).unwrap().is_digit(10) {
                buf.push(self.consume());

                while self.peek(1).is_some() && self.peek(1).unwrap().is_digit(10) {
                    buf.push(self.consume());
                }

                let accumulated: String = buf.iter().map(|a| a.to_string()).collect();

                tokens.push(Token {
                    kind: TokenKind::IntLit,
                    value: Some(accumulated),
                });
                continue;
            } else if self.peek(1).unwrap() == ';' {
                tokens.push(Token {
                    kind: TokenKind::Semi,
                    value: None,
                });
                self.consume();
                buf.clear();
                continue;
            } else if self.peek(1).unwrap().is_whitespace() {
                self.consume();
                continue;
            } else {
                log::error!("Temporary error. Something went wrong though.");
                process::exit(-1);
            }
        }

        // Reset incase used again.
        self.index = 0;

        tokens
    }

    fn peek(&self, ahead: i32) -> Option<char> {
        match self.index + ahead >= self.source.len() as i32 {
            true => return None,
            false => return Some(self.source[self.index as usize]),
        }
    }

    fn consume(&mut self) -> char {
        self.index += 1;

        // log::debug!("{}", self.source[(self.index - 1) as usize]);
        return self.source[(self.index - 1) as usize];
    }
}
