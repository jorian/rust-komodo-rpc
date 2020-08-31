use std::{io, error, fmt};
use std::num::ParseIntError;
use komodo_rpc_json::bitcoin::hashes::core::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    JsonRPC(jsonrpc::Error),
    IOError(io::Error),
    ParseIntError(ParseIntError),
    InvalidConfigFile
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ParseIntError(ref e) => Some(e),
            Error::JsonRPC(ref e) => Some(e),
            Error::IOError(ref e) => Some(e),
            Error::InvalidConfigFile => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ParseIntError(ref e) => write!(f, "Parse error: {}", e),
            Error::JsonRPC(ref e) => write!(f, "RPC error: {}", e),
            Error::IOError(ref e) => write!(f, "IO error: {}", e),
            Error::InvalidConfigFile => write!(f, "Error in config file")
        }
    }
}

impl From<jsonrpc::Error> for Error {
    fn from(e: jsonrpc::Error) -> Error {
        Error::JsonRPC(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseIntError(e)
    }
}


