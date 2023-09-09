use thiserror::*;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Something went wrong and the reason is currently unknown.")]
    Unknown,
    #[error("An invalid expression was detected.")]
    InvalidExpression,
}
