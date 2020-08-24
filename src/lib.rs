#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod client {
    // extern crate jsonrpc;

    use jsonrpc;
    use std::{io, result};
    use crate::error::Error;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Clone, Debug)]
    pub enum Auth {
        UserPass(String, String),
        ConfigFile(path),
    }

    impl Auth {
        fn get(self) -> Result<(String, String)> {
            match self {
                Auth::ConfigFile(path) => {
                    // read config file based on system OS.

                    Ok(("a".to_string(), "b".to_string()))
                },
                Auth::UserPass(rpcuser, rpcpass) => Ok((rpcuser, rpcpass))
            }
        }
    }

    pub struct Client {
        client: jsonrpc::client::Client,
    }

    impl Client {
        pub fn new(url: String, auth: Auth) -> Result<Self> {
            let (rpcuser, rpcpass) = auth.get()?;
            Ok(Client {
                client: jsonrpc::client::Client::new(url, user, pass)
            })
        }
    }
}

mod error {
    use std::io;

    #[derive(debug)]
    pub enum Error {
        JsonRPC(jsonrpc::Error),
        IOError(io::Error)
    }

    impl From<jsonrpc::Error> for Error {
        fn from(e: jsonrpc::Error) -> Error {
            Error::JsonRPC(e)
        }
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Error {
            Error::Io(e)
        }
    }
}