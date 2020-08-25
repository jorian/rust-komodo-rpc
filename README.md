# rust-komodo-rpc

A continuation of komodorpc-rust-client, heavily inspired by 

### Goals
- Learn Rust
  - workspaces
- Simplify codebase
- Do more than just be a RPC lib

### Questions
- Why is there a Queryable trait?
- `#[serde(with = "bitcoin::util::amount::serde::as_btc")]` ?
- `pub mod serde_hex` returns a `FromHex`, which is a trait. But the trait
doesn't specifically say it's a Vec<u8>. How does this work? 
Is it because `FromHex` is `Sized` it could be anything, 
as long as it is `Sized`? 

### TODO
- Localize coin installations
- Error types
- 