#![crate_name = "komodo_rpc"]
#![crate_type = "rlib"]

#[allow(unused)]
#[macro_use] // `macro_use` is needed for v1.24.0 compilation.
extern crate serde;
extern crate serde_json;

pub extern crate jsonrpc;

pub extern crate komodo_rpc_json;
pub use komodo_rpc_json as json;
pub use json::bitcoin;

mod error;
mod client;

pub use client::*;
pub use error::Error;