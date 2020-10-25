use std::fmt;
use std::error;
use std::str;

const SUC_PREFIX: char = '$';
const ERR_PREFIX: char = '!';

#[derive(PartialEq, Debug)]
pub enum Response {
    Success,
    SuccessWithResult(Vec<u8>),
    Error(Vec<u8>),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Response::Success => write!(f, "Ok"),
            Response::SuccessWithResult(res) | Response::Error(res) => {
                match str::from_utf8(&res) {
                    Ok(s) => write!(f, "\"{}\"", s),
                    Err(_) => write!(f, "Recieved non-UTF8 response"),
                }
            },
        }
    }
}

pub fn parse_response(raw: &[u8]) -> Result<Response, ParseError>{
    if 0 < raw.len() {
        match raw[0] as char {
            prefix @ SUC_PREFIX | prefix @ ERR_PREFIX => { 
                let args: Vec<&[u8]>;

                match parse_for_args(&raw[1..]) {
                    Ok(a) => args = a,
                    Err(e) =>  return Err(e),
                }

                match prefix {
                    SUC_PREFIX if 0 < args.len() => return Ok(Response::SuccessWithResult(args[0].to_vec())),
                    SUC_PREFIX => return Ok(Response::Success),
                    ERR_PREFIX if 0 < args.len() => return Ok(Response::Error(args[0].to_vec())),
                    ERR_PREFIX => return Err(ParseError::MissingArgument),
                    _ => return Err(ParseError::InvalidPrefix(String::from(prefix))),
                }
            }, 
            _ => return Err(ParseError::InvalidPrefix(String::from(raw[0] as char ))),
        };
    } else {
        Err(ParseError::EmptyResponse)
    }
}

fn parse_for_args(raw: &[u8]) -> Result<Vec<&[u8]>, ParseError> {
    let mut args: Vec<&[u8]> = Vec::new();
    let mut mut_raw = raw;

    while mut_raw.len() > 0 {

        if mut_raw[0] as char == '\n' {
            break;
        }

        let start_index: usize;
        let end_index: usize;

        match chunk_arg(mut_raw) {
            Ok((start, end)) => {
                start_index = start;
                end_index = end;
            },
            Err(e) => return Err(e),
        }

        args.push(&mut_raw[start_index..end_index]);

        mut_raw = &mut_raw[end_index..];
    }

    Ok(args)
}

fn chunk_arg(raw: &[u8]) -> Result<(usize, usize), ParseError> {
    let mut arg_size_str = String::new();
    let mut i = 0;

    while i < raw.len() {
        match raw[i] as char {
            '\t' => {
                i += 1;
                break;
            },
            '0'..='9' => arg_size_str.push(raw[i] as char),
            _ => return Err(ParseError::InvalidArgumentSize),
        }

        i += 1;
    }

    let arg_size: usize;
    match arg_size_str.parse::<usize>() {
        Ok(size) => arg_size = size,
        Err(_) => return Err(ParseError::InvalidArgumentSize),
    }
    
    if raw[i..].len() < arg_size {
        return Err(ParseError::ArgumentSizeTooBig);
    }

    Ok((i, i + arg_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success_response() {
        let response = b"$\n";

        assert_eq!(parse_response(response).unwrap(), Response::Success);
    } 

    #[test]
    fn parse_success_response_with_result() {
        let response = b"$3\tfoo\n";

        assert_eq!(parse_response(response).unwrap(), Response::SuccessWithResult(b"foo".to_vec()));
    }

    #[test]
    fn parse_error_response() {
        let response = b"!6\tFooBar\n";

        assert_eq!(parse_response(response).unwrap(), Response::Error(b"FooBar".to_vec()));
    }

    #[test]
    fn parse_invalid_prefix() {
        let response = b"c3\tfoo\n";

        assert_eq!(parse_response(response), Err(ParseError::InvalidPrefix));
    }

    #[test]
    fn parse_empty_response() {
        let response = b"";

        assert_eq!(parse_response(response), Err(ParseError::InvalidPrefix));
    }

    #[test]
    fn parse_error_missing_argument() {
        let response = b"!\n";

        assert_eq!(parse_response(response), Err(ParseError::MissingArgument));
    }

    #[test]
    fn parse_invalid_argument_size() {
        let response = b"$abc\tdef\n";

        assert_eq!(parse_response(response), Err(ParseError::InvalidArgumentSize));
    }

    #[test]
    fn parse_argument_size_too_big() {
        let response = b"$100\tfoo\n";

        assert_eq!(parse_response(response), Err(ParseError::ArgumentSizeTooBig));
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidPrefix(String),
    MissingArgument,
    ArgumentSizeTooBig,
    InvalidArgumentSize,
    EmptyResponse,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidPrefix(ref pref) => write!(f, "invalid prefix \"{}\"", pref.escape_debug()),
            ParseError::MissingArgument => write!(f, "missing argument"),
            ParseError::ArgumentSizeTooBig => write!(f, "argument size larger than remaining bytes"),
            ParseError::InvalidArgumentSize => write!(f, "argument size invalid"),
            ParseError::EmptyResponse => write!(f, "response is empty"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
       None
    }
}
