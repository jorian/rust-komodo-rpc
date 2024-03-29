use komodo_rpc_json::komodo;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::{error, fmt, io};
// use komodo_rpc_json::komodo::util::amount::ParseAmountError;

#[derive(Debug)]
pub enum Error {
    JsonRPC(jsonrpc::Error),
    IOError(io::Error),
    ParseIntError(ParseIntError),
    InvalidConfigFile,
    Json(serde_json::error::Error),
    KMDError(String),
    InvalidAmount(komodo::util::amount::ParseAmountError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ParseIntError(ref e) => Some(e),
            Error::JsonRPC(ref e) => Some(e),
            Error::IOError(ref e) => Some(e),
            Error::InvalidConfigFile => None,
            Error::Json(ref e) => Some(e),
            Error::KMDError(_) => None,
            Error::InvalidAmount(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ParseIntError(ref e) => write!(f, "Parse error: {}", e),
            Error::JsonRPC(ref e) => write!(f, "RPC error: {}", e),
            Error::IOError(ref e) => write!(f, "IO error: {}", e),
            Error::InvalidConfigFile => write!(f, "Error in config file"),
            Error::Json(ref e) => write!(f, "JSON error: {}", e),
            Error::KMDError(ref e) => write!(f, "KMD daemon error: {}", e),
            Error::InvalidAmount(ref e) => write!(f, "invalid amount: {}", e),
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

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::Json(e)
    }
}

impl From<komodo::util::amount::ParseAmountError> for Error {
    fn from(e: komodo::util::amount::ParseAmountError) -> Error {
        Error::InvalidAmount(e)
    }
}
