use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Write};
use crate::error::{ErrorKind, TamariError};

pub struct Client {
    stream: TcpStream,
} 

impl Client {
    pub fn connect(host_name: &str, port: u16) -> Result<Client, TamariError> {
        match TcpStream::connect((host_name, port)) {
            Ok(stream) => return Ok(Client{ stream }),
            Err(e) => return Err(TamariError::new(ErrorKind::Connection(e))),
        };
    }

    pub fn get(&mut self, key: &str) {
        let mut request: Vec<u8> = Vec::new();
        let key_len = key.len();

        write!(&mut request, "={}\t{}\n", key_len, key);

        if let Ok(_) = self.stream.write(b"get request") {
            println!("Get request success");
        } else {
            println!("Get request failed with error");
        }
    }
}