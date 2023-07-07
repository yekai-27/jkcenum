use thiserror::Error;


#[derive(Debug, Error)]
pub enum FromStrParseError {
    #[error("invalid str: `{0}`")]
    InvalidStr(String),
}
