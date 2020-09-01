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