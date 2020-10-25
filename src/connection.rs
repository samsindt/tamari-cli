use std::net;
use crate::error::{TamariError, ErrorKind};
use std::io::{Write, BufRead, BufReader, BufWriter};

use std::str;

pub trait Connection: {
    fn read(&mut self) -> Result<Vec<u8>, TamariError>;
    fn write(&mut self, buffer: &[u8]) -> Result<(), TamariError>;
}

pub struct TcpConnection {
    stream: net::TcpStream,
}

impl Connection for TcpConnection {
    fn read(&mut self) -> Result<Vec<u8>, TamariError> {
        
        let mut reader = BufReader::new(&self.stream);
        let mut response = String::new();

        match reader.read_line(&mut response) {
            Ok(_) => return Ok(response.as_bytes().to_vec()), // maybe trim here too
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e)))
        }
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), TamariError> {
        let mut writer = BufWriter::new(&self.stream);
        match writer.write_all(buffer) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        }
    }
}

impl TcpConnection {
    pub fn new(addr: &str, port: u16) -> Result<Self, TamariError> {
        match net::TcpStream::connect((addr, port)) {
            Ok(stream) => return Ok(TcpConnection { stream: stream }),
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        };
    }
}