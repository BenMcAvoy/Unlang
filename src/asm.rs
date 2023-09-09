use crate::lexer::{Token, TokenKind};

pub fn tokens_to_asm(tokens: Vec<Token>) -> String {
    let mut asm = String::new();

    asm.push_str("global _start\n");
    asm.push_str("_start:\n");

    // TODO: Cleanup
    for (index, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Exit {
            if tokens[index + 1].kind == TokenKind::IntLit {
                log::debug!("IntLit");
                if index + 1 < tokens.len() && tokens[index + 2].kind == TokenKind::Semi {
                    log::debug!("Semi");
                    asm.push_str("    mov rax, 60\n");
                    asm.push_str(&format!("    mov rdi, {}\n", tokens[index + 1].value.clone().unwrap()));
                    asm.push_str("    syscall");
                }
            }
        }
    }

    asm
}
