# cex_trading


[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)

Rust Library for the [Cex.io API](https://cex.io/rest-api)

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
cex_trading = { git = "https://github.com/isvforall/cex_trading.git" }
```

## Example

```rust
use cex_trading::api::CexAPI;
use cex_trading::models::Symbol;

const CEX_USERID: &str = ""; // your user id
const CEX_API_KEY: &str = ""; // your api key
const CEX_API_SECRET: &str = ""; // your api secret

fn main() {
    // Public API calls
    println!("{:?}", CexAPI::last_price(Symbol::BTC, Symbol::USD));

    // Private API calls
    let cex = CexAPI::new(CEX_USERID, CEX_API_KEY, CEX_API_SECRET);
    println!("{:?}", cex.balance());
}

```

## License

MIT licensed. See the LICENSE file for details.