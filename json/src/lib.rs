#![crate_name = "komodo_rpc_json"]
#![crate_type = "rlib"]

pub extern crate bitcoin;

#[allow(unused)]
#[macro_use] // `macro_use` is needed for v1.24.0 compilation.
extern crate serde;
extern crate serde_json;
