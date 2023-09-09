use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    index: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
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
