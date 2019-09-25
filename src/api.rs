use crate::models::*;

use crate::request::*;

use std::collections::HashMap;
use std::time::SystemTime;

use ring::hmac;

use hex::encode;

#[derive(Copy, Clone)]
pub struct CexAPI {
    cex_userid: &'static str,
    cex_api_key: &'static str,
    cex_api_secret: &'static str,
}

impl CexAPI {
    pub fn new(
        cex_userid: &'static str,
        cex_api_key: &'static str,
        cex_api_secret: &'static str,
    ) -> Self {
        return CexAPI {
            cex_userid: cex_userid,
            cex_api_key: cex_api_key,
            cex_api_secret: cex_api_secret,
        };
    }

    fn get_signature(self) -> HashMap<String, String> {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            .to_string();

        let signed_key = hmac::Key::new(ring::hmac::HMAC_SHA256, &self.cex_api_secret.as_bytes());

        let mut context = hmac::Context::with_key(&signed_key);
        context.update(&nonce.as_bytes());
        context.update(&self.cex_userid.as_bytes());
        context.update(&self.cex_api_key.as_bytes());

        let signature = encode(context.sign().as_ref()).to_uppercase();

        let mut map = HashMap::new();
        map.insert("key".to_string(), self.cex_api_key.to_string());
        map.insert("signature".to_string(), signature);
        map.insert("nonce".to_string(), nonce);
        return map;
    }

    pub fn currency_limits() -> Result<CurrencyLimitsResult, reqwest::Error> {
        make_get_request("currency_limits/")
    }

    pub fn ticker(symbol1: Symbol, symbol2: Symbol) -> Result<TickerResult, reqwest::Error> {
        make_get_request_symbols("ticker/", symbol1, symbol2)
    }

    pub fn tickers(symbols: Vec<Symbol>) -> Result<TickerMarketsResult, reqwest::Error> {
        let url = symbols_to_string(symbols);
        make_get_request(format!("{}{}", "tickers/", url).as_str())
    }

    pub fn last_price(symbol1: Symbol, symbol2: Symbol) -> Result<LastPriceResult, reqwest::Error> {
        make_get_request_symbols("last_price/", symbol1, symbol2)
    }

    pub fn last_prices(symbols: Vec<Symbol>) -> Result<LastPriceMarketsResult, reqwest::Error> {
        let url = symbols_to_string(symbols);
        make_get_request(format!("{}{}", "last_prices/", url).as_str())
    }

    pub fn price_stats(
        symbol1: Symbol,
        symbol2: Symbol,
        last_hours: i32,
        max_resp_arr_size: i32,
    ) -> Result<Vec<PriceStatsResult>, reqwest::Error> {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("lastHours".to_string(), last_hours.to_string());
        params.insert("maxRespArrSize".to_string(), max_resp_arr_size.to_string());

        make_post_request_symbols(HashMap::new(), "price_stats/", symbol1, symbol2, params)
    }

    pub fn ohlcv(
        date: &str,
        symbol1: Symbol,
        symbol2: Symbol,
    ) -> Result<OhlcvResult, reqwest::Error> {
        let url = format!("ohlcv/hd/{}/{}/{}", date, symbol1, symbol2);
        make_get_request(&url)
    }

    pub fn order_book(symbol1: Symbol, symbol2: Symbol) -> Result<OrderBookResult, reqwest::Error> {
        make_get_request_symbols("order_book/", symbol1, symbol2)
    }

    pub fn trade_history(
        symbol1: Symbol,
        symbol2: Symbol,
    ) -> Result<Vec<TradeHistoryResult>, reqwest::Error> {
        make_get_request_symbols("trade_history/", symbol1, symbol2)
    }

    pub fn convert(
        symbol1: Symbol,
        symbol2: Symbol,
        amnt: i32,
    ) -> Result<ConvertResult, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("amnt".to_string(), amnt.to_string());

        make_post_request_symbols(HashMap::new(), "convert/", symbol1, symbol2, params)
    }

    pub fn balance(self) -> Result<BalanceResult, reqwest::Error> {
        make_post_request(self.get_signature(), "balance/", HashMap::new())
    }

    pub fn open_orders(self) -> Result<Vec<OpenOrderResult>, reqwest::Error> {
        make_post_request(self.get_signature(), "open_orders/", HashMap::new())
    }

    pub fn active_orders_status(
        self,
        order_list: Vec<String>,
    ) -> Result<ActiveOrderStatusResult, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("order_list".to_string(), format!("{:?}", order_list));

        make_post_request(self.get_signature(), "active_orders_status/", params)
    }

    pub fn cancel_order(self, id: i64) -> Result<bool, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        make_post_request(self.get_signature(), "cancel_order/", params)
    }

    pub fn cancel_orders_by_pair(
        self,
        symbol1: Symbol,
        symbol2: Symbol,
    ) -> Result<CancelOrdersByPairResult, reqwest::Error> {
        make_post_request_symbols(
            self.get_signature(),
            "cancel_orders/",
            symbol1,
            symbol2,
            HashMap::new(),
        )
    }

    pub fn place_order(
        self,
        symbol1: Symbol,
        symbol2: Symbol,
        r#type: String,
        amount: f64,
        price: f64,
    ) -> Result<PlaceOrderResult2, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), r#type.to_string());
        params.insert("amount".to_string(), amount.to_string());
        params.insert("price".to_string(), price.to_string());

        make_post_request_symbols(
            self.get_signature(),
            "place_order/",
            symbol1,
            symbol2,
            params,
        )
    }

    pub fn get_order(self, id: i64) -> Result<GetOrderResult, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        make_post_request(self.get_signature(), "get_order/", params)
    }

    pub fn get_order_tx(self, id: i64) -> Result<GetOrderTxResult, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        make_post_request(self.get_signature(), "get_order_tx/", params)
    }

    pub fn get_address(self, currency: Symbol) -> Result<GetAddressResult, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency.to_string());

        make_post_request(self.get_signature(), "get_address/", params)
    }

    pub fn get_myfee(self) -> Result<GetMyfeeResult, reqwest::Error> {
        make_post_request(self.get_signature(), "get_myfee/", HashMap::new())
    }

    pub fn get_marginal_fee(
        self,
        symbol1: Symbol,
        symbol2: Symbol,
    ) -> Result<GetMarginalFeeResult, reqwest::Error> {
        make_post_request_symbols(
            self.get_signature(),
            "get_marginal_fee/",
            symbol1,
            symbol2,
            HashMap::new(),
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::api::CexAPI;
    use crate::models::Symbol;

    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize)]
    struct Credentials {
        CEX_USERID: String,
        CEX_API_KEY: String,
        CEX_API_SECRET: String,
    }

    // Public API calls
    #[test]
    fn currency_limits_test() {
        assert!(CexAPI::currency_limits().is_ok());
    }
    #[test]
    fn ticker_test() {
        assert!(CexAPI::ticker(Symbol::BTC, Symbol::USD).is_ok());
    }

    #[test]
    fn tickers_test() {
        assert!(CexAPI::tickers(vec![Symbol::BTC, Symbol::ETH, Symbol::XRP]).is_ok());
    }

    #[test]
    fn last_price_test() {
        assert!(CexAPI::last_price(Symbol::BTC, Symbol::USD).is_ok());
    }

    #[test]
    fn last_prices_test() {
        assert!(CexAPI::last_prices(vec![Symbol::BTC, Symbol::ETH, Symbol::XRP]).is_ok());
    }

    #[test]
    fn convert_test() {
        assert!(CexAPI::convert(Symbol::BTC, Symbol::USD, 1).is_ok());
    }
    #[test]
    fn price_stats_test() {
        assert!(CexAPI::price_stats(Symbol::BTC, Symbol::USD, 5, 10).is_ok());
    }
    #[test]
    fn ohlcv_test() {
        assert!(CexAPI::ohlcv("20190520", Symbol::BTC, Symbol::USD).is_ok());
    }
    #[test]
    fn order_book_test() {
        assert!(CexAPI::order_book(Symbol::BTC, Symbol::USD).is_ok());
    }
    #[test]
    fn trade_history_test() {
        assert!(CexAPI::trade_history(Symbol::BTC, Symbol::USD).is_ok());
    }

    // Private API calls
    lazy_static! {
        static ref f: String =
            fs::read_to_string("/home/isvforall/credentials.toml").expect("File not found");
        static ref credentials: Credentials = toml::from_str(f.as_str()).unwrap();
        static ref CEX_USERID: &'static str = credentials.CEX_USERID.as_str();
        static ref CEX_API_KEY: &'static str = credentials.CEX_API_KEY.as_str();
        static ref CEX_API_SECRET: &'static str = credentials.CEX_API_SECRET.as_str();
        pub static ref cex_api: CexAPI = CexAPI::new(*CEX_USERID, *CEX_API_KEY, *CEX_API_SECRET);
    }

    #[test]
    fn balance_test() {
        assert!(cex_api.balance().is_ok());
    }
    #[test]
    fn open_orders_test() {
        assert!(cex_api.open_orders().is_ok());
    }
    // TODO
    // #[test]
    fn open_orders_pair_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn active_order_status_test() {
        assert!(cex_api.active_orders_status(vec!["".to_string()]).is_ok());
    }
    // TODO
    // #[test]
    fn archived_order_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn cancel_order_test() {
        assert!(cex_api.cancel_order(1111111111).is_ok());
    }
    #[test]
    fn cancel_order_by_pair_test() {
        assert!(cex_api
            .cancel_orders_by_pair(Symbol::BTC, Symbol::USD)
            .is_ok());
    }
    // #[test]
    fn place_order_test() {
        assert!(cex_api
            .place_order(Symbol::BTC, Symbol::USD, "buy".to_string(), 0.42, 0.42)
            .is_ok());
    }
    // TODO
    // #[test]
    fn get_order_test() {
        assert!(cex_api.get_order(111111111111).is_ok());
    }
    // TODO
    // #[test]
    fn get_order_tx_test() {
        assert!(cex_api.get_order_tx(111111111).is_ok());
    }

    #[test]
    fn get_address_test() {
        assert!(cex_api.get_address(Symbol::BTC).is_ok());
    }
    #[test]
    fn get_myfee_test() {
        assert!(cex_api.get_myfee().is_ok());
    }
    // TODO
    // #[test]
    fn cancel_replace_order_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn open_position_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn get_position_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn open_position_by_symbol_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn close_position_test() {
        assert!(false);
    }
    // TODO
    // #[test]
    fn archived_positions_test() {
        assert!(false);
    }
    #[test]
    fn get_marginal_fee_test() {
        assert!(cex_api.get_marginal_fee(Symbol::BTC, Symbol::USD).is_ok());
    }
}
