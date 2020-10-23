use std::net::TcpStream;
use std::io::{Write, Read};
use crate::error::{ErrorKind, TamariError};
use crate::parser;

pub struct Client {
    stream: TcpStream,
} 

pub use parser::Response;

impl Client {
    pub fn connect(host_name: &str, port: u16) -> Result<Client, TamariError> {
        match TcpStream::connect((host_name, port)) {
            Ok(stream) => return Ok(Client{ stream }),
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        };
    }

    pub fn get<K: ToTamariArg> (&mut self, key: K) -> Result<Response, TamariError>{
        let mut request: Vec<u8> = "=".as_bytes().to_vec();
        let key = key.write_tamari_arg();
        let key_len = key.len();
        // this could be better
        request.append(&mut key_len.to_string().as_bytes().to_vec());
        request.append(&mut "\t".as_bytes().to_vec());
        request.append(&mut key.to_vec());
        request.append(&mut "\n".as_bytes().to_vec());
        self.stream.write(&request)?;

        let mut response_raw: Vec<u8> = Vec::new();
        match self.stream.read(&mut response_raw) {
            Ok(_) => {
                match parser::parse_response(&response_raw) {
                    Ok(resp) => return Ok(resp),
                    Err(e) => return Err(TamariError::new(ErrorKind::Parse(e))),
                }
            },
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        }
    }   
}

pub trait ToTamariArg: Sized {
    fn write_tamari_arg(&self) -> &[u8];
}

impl ToTamariArg for String {
    fn write_tamari_arg(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> ToTamariArg for &'a String {
    fn write_tamari_arg(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> ToTamariArg for &'a str {
    fn write_tamari_arg(&self) -> &[u8] {
        self.as_bytes()
    }
}