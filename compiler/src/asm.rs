use crate::errors::ParseError;
use crate::parser::node;

use std::fmt::Write;

pub struct Generator {
    root: node::Exit,
}

impl Generator {
    pub fn new(root: node::Exit) -> Self {
        Self { root }
    }

    pub fn generate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut asm = String::new();

        asm.push_str("global _start\n");
        asm.push_str("_start:\n");

        let expr_value = match &self.root.expr.int_lit.value {
            Some(v) => v,
            None => {
                return Err(Box::new(ParseError::InvalidExpression))
            }
        };

        write!(asm, "    mov rax, 60\n")?;
        write!(asm, "    mov rdi, {}\n", expr_value)?;
        write!(asm, "    syscall\n")?;

        Ok(asm)
    }
}
