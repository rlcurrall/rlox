use core::result::Result as CoreResult;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct Error {
    line: usize,
    offset: usize,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _offset = self.offset; // @todo - use offset somehow
        let line = self.line;
        let message = self.message.as_str();
        write!(f, "line {line} | Error: {message}")
    }
}

pub type Result<T> = CoreResult<T, RuntimeError>;

pub enum RuntimeError {
    ScanError {
        line: usize,
        offset: usize,
        message: String,
    },
    ParseError(String),
    GeneralError(String),
}

impl RuntimeError {
    pub(crate) fn parse_error(message: String) -> Self {
        Self::ParseError(message)
    }

    pub(crate) fn scan_error(message: String, line: usize, offset: usize) -> Self {
        Self::ScanError {
            message,
            line,
            offset,
        }
    }

    pub(crate) fn general_error(message: &str) -> Self {
        Self::GeneralError(message.into())
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        RuntimeError::GeneralError(value.to_string())
    }
}

impl From<Error> for RuntimeError {
    fn from(value: Error) -> Self {
        RuntimeError::ScanError {
            line: value.line,
            offset: value.offset,
            message: value.message,
        }
    }
}
