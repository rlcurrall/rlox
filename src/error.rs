use core::result::Result as CoreResult;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct Error {
    line: i32,
    offset: i32,
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

pub(crate) type Result<T> = CoreResult<T, RuntimeError>;

pub(crate) enum RuntimeError {
    ParseError(Error),
    OsError(std::io::Error),
}

impl RuntimeError {
    pub(crate) fn parse(message: String, line: i32, offset: i32) -> Self {
        Self::ParseError(Error {
            message,
            line,
            offset,
        })
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        RuntimeError::OsError(value)
    }
}

impl From<Error> for RuntimeError {
    fn from(value: Error) -> Self {
        RuntimeError::ParseError(value)
    }
}
