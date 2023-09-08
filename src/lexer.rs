// TODO: Convert to lexer struct!

pub enum TokenType {
    Return,
    IntLit,
    Semi,
}

pub struct Token {
    kind: TokenType,
    value: Option<String>,
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

    pub fn tokenizer(&mut self) -> Vec<Token> {
        todo!()
    }

    fn peek(&self, ahead: i32) -> Option<char> {
        match self.index + ahead >= self.source.len() as i32 {
            true => return None,
            false => return Some(self.source[self.index as usize])
        }
    }

    fn consume(&mut self) -> char {
        self.index += 1;

        return self.source[(self.index - 1) as usize]
    }
}
