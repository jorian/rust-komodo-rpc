mod client {
    // extern crate jsonrpc;

    use jsonrpc;
    use std::{io, result};
    use crate::error::Error;
    use std::path::{Path, PathBuf};
    use std::collections::HashMap;

    pub type Result<T> = result::Result<T, Error>;


    // #[derive(Clone, Debug)]
    pub enum Auth {
        UserPass(String, String),
        ConfigFile,
    }

    impl Auth {
        fn get(self) -> Result<(String, String)> {
            match self {
                Auth::ConfigFile => {
                    // Linux: /home/$USER/
                    if let Some(mut path) = dirs::home_dir() {
                        match os_info::get().os_type() {
                            OSType::Ubuntu | OSType::Linux => path.push(".komodo"),
                            OSType::Macos | OSType::Windows => path.push("Komodo"),
                            _ => return Err(ApiError::Other(String::from("no komodod installation found")))
                        }
                        config_path = path;
                    } else {
                        return Err(ApiError::Other(String::from("unable to locate home_dir: unsupported OS")))
                    }


                },
                Auth::UserPass(rpcuser, rpcpass) => Ok((rpcuser, rpcpass))
            }
        }
    }

    struct ConfigFile {
        rpc_user: String,
        rpc_password: String,
        rpc_port: u16,
    }

    impl ConfigFile {
        pub fn get_for(chain: &Chain) -> Result<Self> {
            let mut config_path: PathBuf;

            // find location of configuration file:
            match os_info::get().os_type() {
                OSType::Ubuntu | OSType::Linux => {
                    // Linux: /home/$USER/
                    if let Some(mut path) = dirs::home_dir() {
                        path.push(".komodo");
                        config_path = path;
                    } else {
                        return Err(ApiError::Other(String::from("no komodod installation found")))
                    }
                },
                OSType::Macos | OSType::Windows => {
                    // MacOS: /Users/Alice/Library/Application Support
                    // Windows: C:\Users\Alice\AppData\Roaming
                    if let Some(mut path) = dirs::data_dir() {
                        path.push("Komodo");
                        config_path = path;
                    } else {
                        return Err(ApiError::Other(String::from("no komodod installation found")))
                    }
                },
                _ => return Err(ApiError::Other(String::from("unknown or unsupported operating system")))
            }

            // push the actual configuration file:
            match chain {
                Chain::KMD => {
                    config_path.push("komodo.conf"); // conf name is lowercase
                },
                Chain::Custom(chain) => {
                    config_path.push(chain);

                    if !config_path.is_dir() {
                        return Err(ApiError::IO(IOError::from(ErrorKind::NotFound)));
                    }

                    config_path.push(format!("{}.conf", chain.to_string()));
                },
                // assetchain configuration files live in their own directory:
                _ => {
                    config_path.push(chain.to_string());
                    config_path.push(format!("{}.conf", chain.to_string())); // conf name is capitalized
                }
            }

            let contents = fs::read_to_string(config_path.to_str().unwrap())?;

            let map: HashMap<String, String> = contents.as_str()
                .split('\n')
                .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
                .filter(|vec| vec.len() == 2)
                .map(|vec| (
                    vec[0].to_string(),
                    vec[1].to_string()
                ))
                .collect::<HashMap<String, String>>();

            let _rpc_user = map.get("rpcuser").ok_or(ApiError::Config(String::from("No rpcuser in config")))?;
            let _rpc_password = map.get("rpcpassword").ok_or(ApiError::Config(String::from("no rpcpassword in config file")))?;
            let _rpc_port =
                match chain {
                    Chain::KMD => "7771", // KMD doesn't put rpcport in conf file at install
                    _ => map.get("rpcport").ok_or(ApiError::Config(String::from("no rpcport in config file")))?,
                };

            Ok(ConfigFile {
                rpc_user:       _rpc_user.to_owned(),
                rpc_password:   _rpc_password.to_owned(),
                rpc_port:       _rpc_port.parse::<u16>()?
            })
        }
    }

    pub struct Client {
        client: jsonrpc::client::Client,
    }

    impl Client {
        // TODO url must change to coin, and we're trying to find a local installation.
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
        fn get_a_client() {
            let client = Client::new(
                "http://localhost:8332".to_string(),
                Auth::UserPass("123345".to_string(), "54312".to_string())
            );

            assert!(client.is_ok());
        }

        #[test]
        fn get_config() {

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