use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToonMcpError {
    #[error("Encoding error: {0}")]
    Encode(String),

    #[error("Decoding error: {0}")]
    Decode(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
