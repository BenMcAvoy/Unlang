use thiserror::*;

#[derive(Debuf, Error)]
pub enum ParseError {
    #[error("Something went wrong and the reason is currently unknown.")]
    Unknown,
}
