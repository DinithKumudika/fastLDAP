use thiserror::Error;

#[derive(Error, Debug)]
pub enum BerError {
    #[error("Incomplete data")]
    Incomplete,
    #[error("Invalid tag: {0}")]
    InvalidTag(u8),
    #[error("Invalid length")]
    InvalidLength,
    #[error("Unsupported definite length > 4 bytes")]
    UnsupportedLength,
    #[error("String decoding error")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parser error: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum LdapError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("BER decoding error: {0}")]
    Ber(#[from] BerError),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Operation error: {0}")]
    Operation(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("Invalid DN: {0}")]
    InvalidDn(String),
    #[error("Invalid Filter: {0}")]
    InvalidFilter(String),
}

pub type Result<T> = std::result::Result<T, LdapError>;
