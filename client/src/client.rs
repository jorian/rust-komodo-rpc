use jsonrpc;
use std::{io, result, fs};
use crate::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::ErrorKind;

use os_info::Type as OSType;

pub type Result<T> = result::Result<T, Error>;

/// Let the system find a local installation, or supply your own connection details.
#[derive(Clone, Debug)]
pub enum Auth {
    UserPass(String, String, String),
    ConfigFile,
}

#[derive(Debug)]
pub struct ConfigFile {
    rpcuser: String,
    rpcpassword: String,
    rpcport: u16,
}

impl ConfigFile {
    fn get_komodo_installation_folder() -> Result<PathBuf> {
        if let Some(mut path) = dirs::home_dir() {
            match os_info::get().os_type() {
                OSType::Ubuntu | OSType::Linux => path.push(".komodo"),
                OSType::Macos | OSType::Windows => path.push("Komodo"),
                _ => return Err(Error::IOError(io::Error::from(ErrorKind::Other)))
            }

            if !path.is_dir() {
                return Err(Error::IOError(io::Error::from(ErrorKind::NotFound)));
            }

            Ok(path)
        } else {
            return Err(Error::IOError(io::Error::from(ErrorKind::NotFound)))
        }
    }

    pub fn new(coin: &str) -> Result<Self> {
        let mut path = self::ConfigFile::get_komodo_installation_folder().unwrap();
        match coin {
            "KMD" => {
                path.push("komodo.conf");
            },
            _ => {
                path.push(&coin.to_ascii_uppercase());
                path.push(format!("{}.conf", &coin.to_ascii_uppercase()));
            }
        }

        if !path.exists() {
            return Err(Error::IOError(io::Error::from(ErrorKind::NotFound)))
        }

        let contents = fs::read_to_string(path.to_str().unwrap())?;

        let map: HashMap<String, String> = contents.as_str()
            .split('\n')
            .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
            .filter(|vec| vec.len() == 2)
            .map(|vec| (
                vec[0].to_string(),
                vec[1].to_string()
            ))
            .collect::<HashMap<String, String>>();

        let _rpc_user = map.get("rpcuser").ok_or(Error::IOError(io::Error::from(ErrorKind::NotFound)))?;
        let _rpc_password = map.get("rpcpassword").ok_or(Error::IOError(io::Error::from(ErrorKind::NotFound)))?;
        let _rpc_port =
            match coin {
                "KMD" => "7771", // KMD doesn't put rpcport in conf file at install
                _ => map.get("rpcport").ok_or(Error::IOError(io::Error::from(ErrorKind::NotFound)))?,
            };

        Ok(ConfigFile {
            rpcuser:       _rpc_user.to_owned(),
            rpcpassword:   _rpc_password.to_owned(),
            rpcport:       _rpc_port.parse::<u16>()?
        })

    }
}

pub struct Client {
    client: jsonrpc::client::Client,
}

impl Client {
    pub fn new(coin: &str, auth: Auth) -> Result<Self> {
        match auth {
            Auth::ConfigFile => {
                let config = ConfigFile::new(coin)?;
                Ok(Client {
                    client: jsonrpc::client::Client::new(
                        format!("http://127.0.0.1:{}", config.rpcport),
                        Some(config.rpcuser),
                        Some(config.rpcpassword)
                    )
                })
            },
            Auth::UserPass(url, rpcuser, rpcpassword) => {
                Ok(Client {
                    client: jsonrpc::client::Client::new(
                        url,
                        Some(rpcuser),
                        Some(rpcpassword)
                    )
                })
            }
        }
    }
}

impl RpcApi for Client {
    fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        cmd: &str,
        args: &[serde_json::Value]
    ) -> Result<T> {
        let req = self.client.build_request(&cmd, &args);
        let resp = self.client.send_request(&req).map_err(Error::from);

        Ok(resp?.into_result()?)
    }
}

pub trait RpcApi: Sized {
    fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self, cmd: &str,
        args: &[serde_json::Value],
    ) -> Result<T>;
}

#[cfg(test)]
mod tests {
    use crate::client::{ConfigFile, Client, Auth};

    #[test]
    fn get_config() {
        let config_file = ConfigFile::new("KMD").unwrap();
        println!("{:#?}", &config_file);

        let client = Client::new("KMD", Auth::ConfigFile);
        assert!(client.is_ok());

        let client = Client::new("KMD", Auth::UserPass(
            "http://127.0.0.1:7771".to_string(),
            "123kjh12jkl3h1kl23jh".to_string(),
            "213kj4h2kl3j4h23kl4jh".to_string()
        ));
        assert!(client.is_ok());

        let config_file = ConfigFile::new("ILN");
        println!("{:#?}", &config_file);

        let config_file = ConfigFile::new("PIRATE");
        println!("{:#?}", &config_file);
    }
}