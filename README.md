<h1 align="center"> cexio </h1>

<h5 align="center"> Rust Library for the <a href="https://cex.io/rest-api"> Cex.io API</a> </h5>

[![crates.io](https://meritbadge.herokuapp.com/cexio)](https://crates.io/crates/cexio)
[![Downloads from crates.io](https://img.shields.io/crates/d/cexio.svg)](https://crates.io/crates/cexio)
[![doc.rs](https://docs.rs/cexio/badge.svg)](https://docs.rs/cexio/)
[![Dependency status](https://deps.rs/repo/github/isvforall/cexio/status.svg)](https://deps.rs/repo/github/isvforall/cexio)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
cexio = "0.1.2"
```

## Example

```rust
use cexio::api::CexAPI;
use cexio::models::Symbol;

const CEX_USER_ID: &str = "your user id";
const CEX_API_KEY: &str = "your api key";
const CEX_API_SECRET: &str = "your api secret";

fn main() {

    // Public API calls
    println!("{:?}", CexAPI::last_price(Symbol::BTC, Symbol::USD));

    // Private API calls
    let cex = CexAPI::new(CEX_USER_ID, CEX_API_KEY, CEX_API_SECRET);
    println!("{:?}", cex.balance());

}

```

## License

MIT licensed. See the LICENSE file for details.
