use thiserror::Error;


#[derive(Debug, Error)]
pub enum FromStrParseError {
    #[error("invalid str: `{0}`")]
    InvalidStr(String),
}


#[derive(Debug, Error)]
pub enum FromIntParseError {
    #[error("invalid value: `{0}`")]
    InvalidInt(isize),
}
