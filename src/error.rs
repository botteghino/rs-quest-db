use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Forbidden character in name: '{0}'")]
    ForbiddenCharacter(char),
}

#[derive(Debug, Error)]
pub enum LineSenderError {
    #[error("Error converting data to string: {0}")]
    StringConversionError(String),
    #[error("Unterminated line: {0}")]
    UnterminatedLine(String),
    #[error("I/O Error")]
    IOError(#[from] io::Error),
}
