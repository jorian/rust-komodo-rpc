use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    JsonRPC(jsonrpc::Error),
    IOError(io::Error),
    ParseIntError(ParseIntError),
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