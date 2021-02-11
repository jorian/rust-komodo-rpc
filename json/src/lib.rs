#![crate_name = "komodo_rpc_json"]
#![crate_type = "rlib"]

pub extern crate bitcoin;
pub extern crate komodo;

#[allow(unused)]
#[macro_use] // `macro_use` is needed for v1.24.0 compilation.
extern crate serde;
extern crate serde_json;

use serde::{Deserializer, Deserialize, Serialize, Serializer};
use komodo::{PublicKey, PrivateKey};
pub use komodo::Address;
use bitcoin::{PubkeyHash, ScriptHash};
use komodo::util::amount::Amount;
// use bitcoin::hash_types::*;

#[derive(Clone, Debug)]
pub enum PubkeyOrAddress<'a> {
    Address(&'a Address),
    Pubkey(&'a str)
}

impl<'a> serde::Serialize for PubkeyOrAddress<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            PubkeyOrAddress::Address(a) => serde::Serialize::serialize(a, serializer),
            PubkeyOrAddress::Pubkey(p) => serde::Serialize::serialize(p, serializer),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinSupply {
    pub result: String,
    pub coin: String,
    pub height: i32,
    pub supply: f64,
    #[serde(rename = "zfunds")]
    pub z_funds: f64,
    pub sprout: f64,
    pub total: f64,
    #[serde(rename = "lastmonth")]
    pub last_month: Option<f64>,
    #[serde(rename = "monthcoins")]
    pub month_coins: Option<f64>,
    #[serde(rename = "lastquarter")]
    pub last_quarter: Option<f64>,
    #[serde(rename = "quartercoins")]
    pub quarter_coins: Option<f64>,
    #[serde(rename = "lastyear")]
    pub last_year: Option<f64>,
    #[serde(rename = "yearcoins")]
    pub year_coins: Option<f64>,
    pub inflation: Option<f64>,
    #[serde(rename = "blocksperyear")]
    pub blocks_per_year: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub last_notarized_height: u32,
    pub hash: bitcoin::BlockHash,
    pub confirmations: u32,
    #[serde(rename = "rawconfirmations")]
    pub raw_confirmations: u32,
    pub size: u32,
    pub height: u32,
    pub version: u16,
    #[serde(rename = "merkleroot")]
    pub merkle_root: bitcoin::TxMerkleNode,
    #[serde(rename = "segid")]
    pub seg_id: i32,
    #[serde(rename = "finalsaplingroot")]
    pub final_sapling_root: String,
    pub tx: Vec<bitcoin::hash_types::Txid>,
    pub time: u64,
    pub nonce: String,
    pub solution: String,
    pub bits: String,
    pub difficulty: f64,
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    pub anchor: String,
    #[serde(rename = "blocktype")]
    pub block_type: String,
    #[serde(rename = "valuePools")]
    pub value_pools: Vec<ValuePool>,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: Option<bitcoin::BlockHash>,
    #[serde(rename = "nextblockhash")]
    pub next_blockhash: Option<bitcoin::BlockHash>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValuePool {
    pub id: String,
    pub monitored: bool,
    #[serde(rename = "chainValue")]
    pub chain_value: f64,
    #[serde(rename = "chainValueZat")]
    pub chain_value_sat: u64,
    #[serde(rename = "valueDelta")]
    pub value_delta: f64,
    #[serde(rename = "valueDeltaZat")]
    pub value_delta_sat: u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletInfo {
    #[serde(rename = "walletversion")]
    pub wallet_version: u32,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub immature_balance: f64,
    #[serde(rename = "txcount")]
    pub tx_count: u32,
    #[serde(rename = "keypoololdest")]
    pub keypool_oldest: u64,
    #[serde(rename = "keypoolsize")]
    pub keypool_size: u32,
    pub unlocked_until: Option<u32>,
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    // Todo what is this?
    #[serde(rename = "seedfp")]
    pub seed_fp: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CleanedWalletTransactions {
    #[serde(rename = "total_transactons")]
    pub total: u8,
    #[serde(rename = "remaining_transactons")]
    pub remaining: u8,
    #[serde(rename = "removed_transactions")]
    pub removed: u8
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConvertedPassphrase {
    #[serde(rename = "agamapassphrase")]
    passphrase: String,
    address: Address,
    #[serde(rename = "pubkey")]
    public_key: PublicKey,
    #[serde(rename = "privkey")]
    private_key: PrivateKey,
    wif: PrivateKey
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTransactionResult {
    amount: f64,
    fee: Option<f64>,
    rawconfirmations: u32,
    confirmations: u32,
    blockhash: Option<bitcoin::BlockHash>,
    blockindex: u32,
    blocktime: Option<u64>,
    expiryheight: u32,
    txid: bitcoin::Txid,
    walletconflicts: Vec<Option<bitcoin::Txid>>,
    time: u64,
    timereceived: u64,
    vjoinsplit: Vec<Option<GetTransactionVJoinSplit>>,
    hex: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTransactionVJoinSplit {
    anchor: String, // Merkle root
    nullifiers: Vec<Option<String>>,
    commitments: Vec<Option<String>>,
    macs: Vec<Option<String>>,
    vpub_old: f64,
    vpub_new: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTransactionDetails {
    account: String,
    address: Address,
    category: GetTransactionDetailsCategory,
    amount: f64,
    vout: u16,
    fee: Option<f64>,
    size: u32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GetTransactionDetailsCategory {
    Send,
    Receive
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListLockUnspentResult {
    pub txid: bitcoin::Txid,
    pub vout: u16
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListReceivedByAddressResult {
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: Option<bool>,
    pub address: Address,
    account: String,
    #[serde(with = "komodo::util::amount::serde::as_kmd")]
    pub amount: Amount,
    pub confirmations: u32
}