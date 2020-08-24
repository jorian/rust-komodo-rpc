mod client {
    // extern crate jsonrpc;

    use jsonrpc;
    use std::{io, result};
    use crate::error::Error;
    use std::path::{Path, PathBuf};

    pub type Result<T> = result::Result<T, Error>;

    // #[derive(Clone, Debug)]
    pub enum Auth {
        UserPass(String, String),
        ConfigFile(PathBuf),
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
                client: jsonrpc::client::Client::new(url, Some(rpcuser), Some(rpcpass))
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::client::{Client, Auth};

        #[test]
        fn it_works() {
            let client = Client::new(
                "http://localhost:8332".to_string(),
                Auth::UserPass("123345".to_string(), "54312".to_string())
            );

            assert!(client.is_ok());
        }
    }
}

mod error {
    use std::io;

    #[derive(Debug)]
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
            Error::IOError(e)
        }
    }
}