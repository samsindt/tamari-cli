use crate::error::{ErrorKind, TamariError};
use crate::parser;
use crate::connection::Connection;

pub struct Client {
    connection: Box<dyn Connection>,
} 

pub use parser::Response;

impl Client {
    pub fn new(connection: Box<dyn Connection>) -> Self {
        Client { connection: connection }
    }

    pub fn get<K: ToTamariArg> (&mut self, key: K) -> Result<Response, TamariError> {
        let mut request: Vec<u8> = b"=".to_vec();
        let key = key.write_tamari_arg();
        let key_len = key.len();
        // this could be better
        request.append(&mut key_len.to_string().as_bytes().to_vec());
        request.append(&mut b"\t".to_vec());
        request.append(&mut key.to_vec());
        request.append(&mut b"\n".to_vec());

        self.connection.write(&request)?;

        match self.connection.read() {
            Ok(resp_raw) => {
                match parser::parse_response(&resp_raw) {
                    Ok(resp) => return Ok(resp),
                    Err(e) => return Err(TamariError::new(ErrorKind::Parse(e))),
                }
            },
            Err(e) => return Err(e) ,
        }
    }

    pub fn set<K: ToTamariArg, V:ToTamariArg> (&mut self, key: K, value: V) -> Result<Response, TamariError> {
        let mut key = key.write_tamari_arg().to_vec();
        let mut key_len = key.len().to_string().as_bytes().to_vec();
        let mut value = value.write_tamari_arg().to_vec();
        let mut value_len = value.len().to_string().as_bytes().to_vec();

        let mut request: Vec<u8> = b"+".to_vec();

        request.append(&mut key_len);
        request.append(&mut b"\t".to_vec());
        request.append(&mut key);
        request.append(&mut value_len);
        request.append(&mut b"\t".to_vec());
        request.append(&mut value);
        request.append(&mut b"\n".to_vec());

        self.connection.write(&request)?;

        match self.connection.read() {
            Ok(resp_raw) => {
                match parser::parse_response(&resp_raw) {
                    Ok(resp) => return Ok(resp),
                    Err(e) => return Err(TamariError::new(ErrorKind::Parse(e))),
                }
            },
            Err(e) => return Err(e) ,
        }
    }

    pub fn delete<K: ToTamariArg> (&mut self, key: K) -> Result<Response, TamariError> {
        let mut key = key.write_tamari_arg().to_vec();
        let mut key_len = key.len().to_string().as_bytes().to_vec();

        let mut request: Vec<u8> = b"-".to_vec();

        request.append(&mut key_len);
        request.append(&mut b"\t".to_vec());
        request.append(&mut key);
        request.append(&mut b"\n".to_vec());

        self.connection.write(&request)?;

        match self.connection.read() {
            Ok(resp_raw) => {
                match parser::parse_response(&resp_raw) {
                    Ok(resp) => return Ok(resp),
                    Err(e) => return Err(TamariError::new(ErrorKind::Parse(e))),
                }
            },
            Err(e) => return Err(e) ,
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

impl<'a> ToTamariArg for &'a[u8] {
    fn write_tamari_arg(&self) -> &[u8] {
        self
    }
}


mod tests {
    use super::*;
    use std::io;

    struct MockConnection<'a> {
        pub expected_write: &'a[u8],
        pub expected_read: &'a[u8],
    }

    impl<'a> Connection for MockConnection<'a> {
        fn read(&mut self) -> Result<Vec<u8>, TamariError> {
            Ok(self.expected_read.to_vec())
        }

        fn write(&mut self, buffer: &[u8]) -> Result<(), TamariError> {
           if buffer == self.expected_write {
                Ok(())
            } else {
                Err(TamariError::new(ErrorKind::IO(io::Error::new(io::ErrorKind::Other, "invalid write"))))
            }
        }
    }

    #[test]
    fn get_success () {
        let key = b"foo";
        let expected_response = b"bar";

        let connection = MockConnection {
            expected_write: b"=3\tfoo\n",
            expected_read: b"$3\tbar\n",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.get(&key[..]);

        assert!(response.is_ok());

        assert_eq!(response.unwrap(), Response::SuccessWithResult(expected_response.to_vec()));
    }

    #[test]
    fn get_corrupt_response() {
        let key = b"foo";

        let connection = MockConnection {
            expected_write: b"=3\tfoo\n",
            expected_read: b"abcdef",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.get(&key[..]);

        assert!(response.is_err());

        let _expected = TamariError::new(ErrorKind::Parse(parser::ParseError::InvalidPrefix(String::from("a"))));

        assert!(matches!(response.unwrap_err(), _expected));
    }

    #[test]
    fn set_success() {
        let key = b"foo";
        let value = "bar";

        let connection = MockConnection {
            expected_write: b"+3\tfoo3\tbar\n",
            expected_read: b"$\n",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.set(&key[..], &value[..]);

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), Response::Success);
    }

    #[test]
    fn set_corrupt_response() {
        let key = b"foo";
        let value = "bar";

        let connection = MockConnection {
            expected_write: b"+3\tfoo3\tbar\n",
            expected_read: b"abcdef",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.set(&key[..], &value[..]);

        assert!(response.is_err());

        let _expected = TamariError::new(ErrorKind::Parse(parser::ParseError::InvalidPrefix(String::from("a"))));

        assert!(matches!(response.unwrap_err(), _expected));
    }

    #[test]
    fn delete_success() {
        let key = b"foo";
        let expected_response = b"bar";

        let connection = MockConnection {
            expected_write: b"-3\tfoo\n",
            expected_read: b"$3\tbar\n",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.delete(&key[..]);

        assert!(response.is_ok());

        assert_eq!(response.unwrap(), Response::SuccessWithResult(expected_response.to_vec()));
    }

    #[test]
    fn delete_corrupt_response() {
        let key = b"foo";

        let connection = MockConnection {
            expected_write: b"-3\tfoo\n",
            expected_read: b"abcdef",
        };

        let mut client = Client::new(Box::new(connection));

        let response = client.delete(&key[..]);

        assert!(response.is_err());

        let _expected = TamariError::new(ErrorKind::Parse(parser::ParseError::InvalidPrefix(String::from("a"))));

        assert!(matches!(response.unwrap_err(), _expected));
    }
}