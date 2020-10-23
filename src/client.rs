use std::net::TcpStream;
use std::io::{Write, Read};
use crate::error::{ErrorKind, TamariError};

pub struct Client {
    stream: TcpStream,
} 

impl Client {
    pub fn connect(host_name: &str, port: u16) -> Result<Client, TamariError> {
        match TcpStream::connect((host_name, port)) {
            Ok(stream) => return Ok(Client{ stream }),
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        };
    }

    pub fn get<T: ToTamariArg> (&mut self, key: T) -> Result<(), TamariError>{
        let mut request: Vec<u8> = "=".as_bytes().to_vec();
        let key = key.write_tamari_arg();
        let key_len = key.len();
        // this could be better
        request.append(&mut key_len.to_string().as_bytes().to_vec());
        request.append(&mut "\t".as_bytes().to_vec());
        request.append(&mut key.to_vec());
        request.append(&mut "\n".as_bytes().to_vec());
        self.stream.write(&request)?;

        let mut response = [0 as u8; 128];
        match self.stream.read(&mut response) {
            Ok(size) => {
                if 0 < size {
                    match response[0] as char {
                        '\n' => return {
                            println!("success $");
                            Ok(())
                        },
                        '!' => {
                            //determine error type from response and return it
                            println!("failure !");
                        },
                        _ => {
                            //return 'response ambigous' error
                            println!("ambigous prefix");
                        },
                    }
                }
            },
            Err(e) => return Err(TamariError::new(ErrorKind::IO(e))),
        }

        Ok(())
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