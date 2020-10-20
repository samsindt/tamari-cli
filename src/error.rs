use url;
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct TamariError {
    kind: ErrorKind,
}

impl TamariError {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn new(e: ErrorKind) -> TamariError {
        TamariError { kind: e }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Parse(url::ParseError),
    Connection(io::Error),
}


impl fmt::Display for TamariError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::Parse(ref err) => write!(f, "Parse error: {}", err),
            ErrorKind::Connection(ref err) => write!(f, "Connection error: {}", err),
        }
    }
}

impl error::Error for TamariError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            ErrorKind::Parse(ref err) => err.source(),
            ErrorKind::Connection(ref err) => err.source(),
            _ => None,
        }
    }
}

impl From<url::ParseError> for ErrorKind {
    fn from(err: url::ParseError) -> Self {
        ErrorKind::Parse(err)
    }
}