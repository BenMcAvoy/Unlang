use crate::errors::ParseError;

use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    index: i32,
}

pub mod node {
    use crate::lexer::Token;

    #[derive(Default)]
    pub struct Expr {
        pub int_lit: Token,
    }

    #[derive(Default)]
    pub struct Exit {
        pub expr: self::Expr,
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<node::Exit, ParseError> {
        while let Some(token) = self.peek(1) {
            if token.kind == TokenKind::Exit {
                // Dump value out of tokens
                self.consume();

                if let Some(expr) = self.parse_expr() {
                    return Ok(node::Exit { expr });
                } else {
                    return Err(ParseError::InvalidExpression)
                }
            }
        }

        self.index = 0;

        Err(ParseError::Unknown)
    }

    fn parse_expr(&mut self) -> Option<node::Expr> {
        if let Some(token) = self.peek(1) {
            if token.kind == TokenKind::IntLit {
                return Some(node::Expr { int_lit: self.consume() })
            }
        }

        None
    }

    fn peek(&self, ahead: i32) -> Option<Token> {
        match self.index + ahead >= self.tokens.len() as i32 {
            true => return None,
            false => {
                let token = &self.tokens[self.index as usize];
                return Some(token.clone());
            }
        }
    }

    fn consume(&mut self) -> Token {
        self.index += 1;

        return self.tokens[(self.index - 1) as usize].clone();
    }
}
