use jsonrpc;
use std::{io, result, fs};
use crate::error::Error;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::error::Error::IOError;
use std::io::ErrorKind;

use os_info::Type as OSType;

pub type Result<T> = result::Result<T, Error>;


// #[derive(Clone, Debug)]
// pub enum Auth {
//     UserPass(String, String),
//     ConfigFile,
// }
//
// impl Auth {
//     fn get(self, coin: &str) -> Result<(String, String)> {
//
//
//
//
//         Err(Error::IOError)
//     }
// }

#[derive(Debug)]
struct ConfigFile {
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
            dbg!(&path);

            if !path.is_dir() {
                return Err(Error::IOError(io::Error::from(ErrorKind::NotFound)));
            }

            Ok(path)
        } else {
            return Err(Error::IOError(io::Error::from(ErrorKind::NotFound)))
        }
    }

    fn get_config_file(path: &PathBuf, coin: &str) -> Result<Self> {
        let mut path = path.clone();
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

    // pub fn get_for(coin: &str) -> Result<Self> {
    //     match os_info::get().os_type() {
    //         Auth::ConfigFile => {
    //             // Linux: /home/$USER/
    //             if let Some(mut path) = dirs::home_dir() {
    //                 match os_info::get().os_type() {
    //                     OSType::Ubuntu | OSType::Linux => path.push(".komodo"),
    //                     OSType::Macos | OSType::Windows => path.push("Komodo"),
    //                     _ => return Err(ApiError::Other(String::from("no komodod installation found")))
    //                 }
    //                 config_path = path;
    //             } else {
    //                 return Err(ApiError::Other(String::from("unable to locate home_dir: unsupported OS")))
    //             }
    //         },
    //         Auth::UserPass(rpcuser, rpcpass) => Ok((rpcuser, rpcpass))
    //     }
    //
    //     // push the actual configuration file:
    //     match chain {
    //         Chain::KMD => {
    //             config_path.push("komodo.conf"); // conf name is lowercase
    //         },
    //         Chain::Custom(chain) => {
    //             config_path.push(chain);
    //
    //             if !config_path.is_dir() {
    //                 return Err(ApiError::IO(IOError::from(ErrorKind::NotFound)));
    //             }
    //
    //             config_path.push(format!("{}.conf", chain.to_string()));
    //         },
    //         // assetchain configuration files live in their own directory:
    //         _ => {
    //             config_path.push(chain.to_string());
    //             config_path.push(format!("{}.conf", chain.to_string())); // conf name is capitalized
    //         }
    //     }
    //
    //     let contents = fs::read_to_string(config_path.to_str().unwrap())?;
    //
    //     let map: HashMap<String, String> = contents.as_str()
    //         .split('\n')
    //         .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
    //         .filter(|vec| vec.len() == 2)
    //         .map(|vec| (
    //             vec[0].to_string(),
    //             vec[1].to_string()
    //         ))
    //         .collect::<HashMap<String, String>>();
    //
    //     let _rpc_user = map.get("rpcuser").ok_or(ApiError::Config(String::from("No rpcuser in config")))?;
    //     let _rpc_password = map.get("rpcpassword").ok_or(ApiError::Config(String::from("no rpcpassword in config file")))?;
    //     let _rpc_port =
    //         match chain {
    //             Chain::KMD => "7771", // KMD doesn't put rpcport in conf file at install
    //             _ => map.get("rpcport").ok_or(ApiError::Config(String::from("no rpcport in config file")))?,
    //         };
    //
    //     Ok(ConfigFile {
    //         rpc_user:       _rpc_user.to_owned(),
    //         rpc_password:   _rpc_password.to_owned(),
    //         rpc_port:       _rpc_port.parse::<u16>()?
    //     })
    // }
}

pub struct Client {
    client: jsonrpc::client::Client,
}

impl Client {
    // pub fn new(config: ConfigFile) -> Result<Self> {
    //     let (rpcuser, rpcpass) = auth.get(&coin)?;
    //     Ok(Client {
    //         client: jsonrpc::client::Client::new(url, Some(rpcuser), Some(rpcpass))
    //     })
    // }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn get_a_client() {
    //     let config = ConfigFile::get
    //     let client = Client::new();
    //
    //     assert!(client.is_ok());
    // }

    use crate::client::ConfigFile;

    #[test]
    fn get_config() {
        let path = ConfigFile::get_komodo_installation_folder().unwrap();
        let config_file = ConfigFile::get_config_file(&path, "KMD");
        println!("{:#?}", &config_file);

        let config_file = ConfigFile::get_config_file(&path, "ILN");
        println!("{:#?}", &config_file);

        let config_file = ConfigFile::get_config_file(&path, "PIRATE");
        println!("{:#?}", &config_file);

    }
}