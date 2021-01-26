use jsonrpc;
use std::{io, result, fs};
use crate::error::Error;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::io::ErrorKind;

use os_info::Type as OSType;

use crate::{bitcoin, json};
use komodo_rpc_json::bitcoin::hashes::hex::FromHex;
use komodo_rpc_json::{GetTransactionResult};
use komodo_rpc_json::komodo::PrivateKey;
use crate::json::komodo::util::address::AddressType;

pub type Result<T> = result::Result<T, Error>;

fn into_json<T>(val: T) -> Result<serde_json::Value>
    where T: serde::ser::Serialize,
{
    Ok(serde_json::to_value(val)?)
}

/// Shorthand for converting an Option into an Option<serde_json::Value>.
fn opt_into_json<T>(opt: Option<T>) -> Result<serde_json::Value>
    where
        T: serde::ser::Serialize,
{
    match opt {
        Some(val) => Ok(into_json(val)?),
        None => Ok(serde_json::Value::Null),
    }
}

/// Shorthand for `serde_json::Value::Null`.
fn null() -> serde_json::Value {
    serde_json::Value::Null
}

/// Shorthand for an empty serde_json::Value array.
fn empty_arr() -> serde_json::Value {
    serde_json::Value::Array(vec![])
}

/// Shorthand for an empty serde_json object.
fn empty_obj() -> serde_json::Value {
    serde_json::Value::Object(Default::default())
}

/// Handle default values in the argument list
///
/// Substitute `Value::Null`s with corresponding values from `defaults` table,
/// except when they are trailing, in which case just skip them altogether
/// in returned list.
///
/// Note, that `defaults` corresponds to the last elements of `args`.
///
/// ```norust
/// arg1 arg2 arg3 arg4
///           def1 def2
/// ```
///
/// Elements of `args` without corresponding `defaults` value, won't
/// be substituted, because they are required.
fn handle_defaults<'a, 'b>(
    args: &'a mut [serde_json::Value],
    defaults: &'b [serde_json::Value],
) -> &'a [serde_json::Value] {
    assert!(args.len() >= defaults.len());

    // Pass over the optional arguments in backwards order, filling in defaults after the first
    // non-null optional argument has been observed.
    let mut first_non_null_optional_idx = None;
    for i in 0..defaults.len() {
        let args_i = args.len() - 1 - i;
        let defaults_i = defaults.len() - 1 - i;
        if args[args_i] == serde_json::Value::Null {
            if first_non_null_optional_idx.is_some() {
                if defaults[defaults_i] == serde_json::Value::Null {
                    panic!("Missing `default` for argument idx {}", args_i);
                }
                args[args_i] = defaults[defaults_i].clone();
            }
        } else if first_non_null_optional_idx.is_none() {
            first_non_null_optional_idx = Some(args_i);
        }
    }

    let required_num = args.len() - defaults.len();

    if let Some(i) = first_non_null_optional_idx {
        &args[..i + 1]
    } else {
        &args[..required_num]
    }
}

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

        let _rpc_user = map.get("rpcuser").ok_or(Error::InvalidConfigFile)?;
        let _rpc_password = map.get("rpcpassword").ok_or(Error::InvalidConfigFile)?;
        let _rpc_port =
            match coin {
                // KMD doesn't put rpcport in conf file at install, but users could have modified it afterwards.
                "KMD" => {
                    match map.get("rpcport") {
                        Some(port) => port,
                        None => "7771"
                    }
                }
                _ => map.get("rpcport").ok_or(Error::InvalidConfigFile)?,
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

        dbg!(&resp);

        Ok(resp?.into_result()?)
    }
}

// This trait is to be implemented by an implementation of a client, and only the `call` method
// is to be implemented.
// All the other methods are methods that a client can call, which in turn do RPCs to the coin daemon.
// the `for` keyword used in serde is done to let the serde deserializer determine the lifetime of
// anything that is put out, in contrast with letting the function caller determine the lifetime.
// (Higher-Ranked Trait Bounds)
pub trait RpcApi: Sized {
    fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self, cmd: &str,
        args: &[serde_json::Value],
    ) -> Result<T>;

    /// Get block hash at a given height
    fn get_block_hash(&self, height: u64) -> Result<bitcoin::BlockHash> {
        self.call("getblockhash", &[height.into()])
    }

    fn get_coin_supply(&self, height: &str) -> Result<json::CoinSupply> {
        self.call("coinsupply", &[height.into()])
    }

    /// Get a block, based on its hash (later on: and height todo).
    fn get_block(&self, hash: &bitcoin::BlockHash) -> Result<json::Block> {
        let val = serde_json::to_value(hash)?;

        Ok(self.call("getblock", &[val])?)
        // the BTC rpc library explicitly validates the bytes that are returned from the daemon.

        // let hex: String = self.call("getblock", &[val])?;
        // let bytes: Vec<u8> = FromHex::from_hex(&hex)?;
        // let deserialized = bitcoin::consensus::encode::deserialize(&bytes)?;
        //
        // Ok(deserialized)
        // fetch the hex
        // make it a Vec<u8>
        // that data needs to be consensus deserialized, to make sure it is a valid hash.
        // into_json()
    }

    fn ping(&self) -> Result<()> {
        self.call("ping", &[])
    }

    // Label is deprecated and thus not used in the method call.
    // Todo keys are either an address or a pubkey.
    fn add_multi_sig_address(&self, n_required: u8, keys: &[json::PubkeyOrAddress]) -> Result<String> {
        // maximum of 15 in a msig.
        if n_required > 15 {
            return Err(Error::KMDError(String::from("No more than 15 signers in a msig allowed")))
        }

        self.call("addmultisigaddress", &[n_required.into(), into_json(keys)?])
    }

    fn backup_wallet(&self, destination: &str) -> Result<PathBuf> {
        self.call("backupwallet", &[destination.into()]).map(|path: String| PathBuf::from(&path))
    }

    fn clean_wallet_transactions(&self) -> Result<json::CleanedWalletTransactions> { self.call("cleanwallettransactions", &[]) }

    fn convert_passphrase(&self, passphrase: &str) -> Result<json::ConvertedPassphrase> {
        self.call("convertpassphrase", &[passphrase.into()])
    }

    fn dump_privkey(&self, address: json::Address) -> Result<PrivateKey> {
        match address.addr_type {
            AddressType::Shielded => return Err(Error::KMDError(String::from("no support for shielded addresses for this call"))),
            _ => {}
        }
        self.call("dumpprivkey", &[address.to_string().into()])
    }

    fn get_balance(&self, minconf: Option<usize>, include_watchonly: Option<bool>) -> Result<Amount> {

    }

    fn import_privkey(&self, privkey: &str) {
        unimplemented!()
    }

    fn get_transaction(&self, txid: &bitcoin::Txid, include_watch_only: Option<bool>) -> Result<GetTransactionResult> {
        let mut args = [into_json(txid)?, opt_into_json(include_watch_only)?];
        self.call("gettransaction", handle_defaults(&mut args, &[null()]))
    }

    fn get_unconfirmed_balance(&self) -> Result<f64> {
        self.call("getunconfirmedbalance", &[])
    }

    fn get_wallet_info(&self) -> Result<json::WalletInfo> {
        self.call("getwalletinfo", &[])
    }

    fn get_new_address(&self) -> Result<json::Address> { self.call("getnewaddress", &[])}

    fn set_tx_fee(&self, amount: f64) -> Result<bool> {
        self.call("settxfee", &[amount.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{ConfigFile, Client, Auth};

    // todo https://github.com/iredelmeier/filesystem-rs/blob/master/src/lib.rs

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