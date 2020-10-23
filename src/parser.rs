use std::fmt;

const SET_PREFIX: char = '+';
const GET_PREFIX: char = '=';
const DEL_PREFIX: char = '-';
const TRA_PREFIX: char = '*';

const SUC_PREFIX: char = '$';
const ERR_PREFIX: char = '!';

#[derive(PartialEq, Debug)]
pub enum Response<'a> {
    Success,
    SuccessResult(&'a[u8]),
    Error(&'a[u8]),
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
                    SUC_PREFIX if 0 < args.len() => return Ok(Response::SuccessResult(args[0])),
                    SUC_PREFIX => return Ok(Response::Success),
                    ERR_PREFIX if 0 < args.len() => return Ok(Response::Error(args[0])),
                    ERR_PREFIX => return Err(ParseError::MissingArgument),
                    _ => return Err(ParseError::InvalidPrefix),
                }
            }, 
            _ => return Err(ParseError::InvalidPrefix),
        }
    } else {
        Err(ParseError::InvalidPrefix) // maybe should be EmptyResponse error instead if errors are seperated between responses and requests
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

        assert_eq!(parse_response(response).unwrap(), Response::SuccessResult(b"foo"));
    }

    #[test]
    fn parse_error_response() {
        let response = b"!6\tFooBar\n";

        assert_eq!(parse_response(response).unwrap(), Response::Error(b"FooBar"));
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
    InvalidPrefix,
    MissingArgument,
    ArgumentSizeTooBig,
    InvalidArgumentSize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidPrefix => write!(f, "invalid prefix"),
            MissingArgument => write!(f, "missing argument"),
            ArgumentSizeTooBig => write!(f, "argument size larger than remaining bytes"),
            InvalidArgumentSize => write!(f, "argument size invalid"),
        }
    }
}