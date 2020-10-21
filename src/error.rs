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
    IO(io::Error),
}


impl fmt::Display for TamariError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::IO(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for TamariError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            ErrorKind::IO(ref err) => err.source(),
            _ => None,
        }
    }
}

impl From<io::Error> for TamariError {
    fn from(err: io::Error) -> Self {
        TamariError { kind: ErrorKind::IO(err) }
    }
}