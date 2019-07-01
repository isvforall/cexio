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
