use crate::lexer::{Token, TokenKind};
use std::fmt::Write;

pub fn tokens_to_asm(tokens: Vec<Token>) -> Result<String, Box<dyn std::error::Error>> {
    let mut asm = String::new();

    asm.push_str("global _start\n");
    asm.push_str("_start:\n");

    for (index, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Exit {
            if tokens[index + 1].kind == TokenKind::IntLit {
                log::debug!("IntLit");
                if index + 1 < tokens.len() && tokens[index + 2].kind == TokenKind::Semi {
                    log::debug!("Semi");

                    let code = tokens[index + 1].value.clone().unwrap_or(String::from("1"));

                    write!(asm, "    mov rax, 60\n")?;
                    write!(asm, "    mov rdi, {}\n", code)?;
                    write!(asm, "    syscall")?;
                }
            }
        }
    }

    Ok(asm)
}
