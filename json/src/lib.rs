#![crate_name = "komodo_rpc_json"]
#![crate_type = "rlib"]

pub extern crate bitcoin;

#[allow(unused)]
#[macro_use] // `macro_use` is needed for v1.24.0 compilation.
extern crate serde;
extern crate serde_json;


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
    pub rawconfirmations: u32,
    pub size: u32,
    pub height: u32,
    pub version: u16,
    pub merkleroot: bitcoin::TxMerkleNode,
    pub segid: i32,
    pub finalsaplingroot: String,
    pub tx: Vec<bitcoin::hash_types::Txid>,
    pub time: u64,
    pub nonce: String,
    pub solution: String,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub anchor: String,
    pub blocktype: String,
    pub valuePools: Vec<ValuePool>,
    pub previousblockhash: Option<bitcoin::BlockHash>,
    pub nextblockhash: Option<bitcoin::BlockHash>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValuePool {
    pub id: String,
    pub monitored: bool,
    pub chainValue: f64,
    pub chainValueZat: u64,
    pub valueDelta: f64,
    pub valueDeltaZat: u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletInfo {
    pub walletversion: u32,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub immature_balance: f64,
    pub txcount: u32,
    pub keypoololdest: u64,
    pub keypoolsize: u32,
    pub unlocked_until: u32,
    pub paytxfee: f64,
    // Todo what is this?
    pub seedfp: String
}