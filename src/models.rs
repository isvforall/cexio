use serde::{Deserialize, Serialize};

use strum_macros::Display;

#[derive(Display)]
pub enum Symbol {
    ADA,
    ATOM,
    BAT,
    BCH,
    BF1,
    BTC,
    BTG,
    BTT,
    DASH,
    DVC,
    ETH,
    EUR,
    GAS,
    GHS,
    GUSD,
    IXC,
    LTC,
    NEO,
    NMC,
    ONG,
    ONT,
    RUB,
    TRX,
    USD,
    USDC,
    USDT,
    XLM,
    XRP,
    ZEC,
}

pub fn symbols_to_string(symbols: Vec<Symbol>) -> String {
    symbols
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/")
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsPair {
    pub symbol1: String,
    pub symbol2: String,
    pub minLotSize: f64,
    pub minLotSizeS2: f64,
    // pub maxLotSize: f64,
    pub minPrice: String,
    pub maxPrice: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsData {
    pub pairs: Vec<CurrencyLimitsPair>,
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsResult {
    pub e: String,
    pub ok: String,
    pub data: CurrencyLimitsData,
}
#[derive(Deserialize, Debug)]
pub struct TickerResult {
    pub timestamp: String,
    pub low: String,
    pub high: String,
    pub last: String,
    pub volume: String,
    pub volume30d: String,
    pub bid: f32,
    pub ask: f32,
}

#[derive(Debug, Deserialize)]
pub struct TickerMarketsData {
    pub volume: String,
    pub last: String,
    pub timestamp: String,
    pub bid: f64,
    pub high: String,
    pub ask: f64,
    pub low: String,
    pub pair: String,
    pub volume30d: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerMarketsResult {
    pub ok: String,
    pub e: String,
    pub data: Vec<TickerMarketsData>,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceResult {
    pub lprice: String,
    pub curr1: String,
    pub curr2: String,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceData {
    pub symbol1: String,
    pub symbol2: String,
    pub lprice: String,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceMarketsResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<LastPriceData>,
}

#[derive(Debug, Deserialize)]
pub struct ConvertResult {
    pub amnt: f64,
}

#[derive(Debug, Deserialize)]
pub struct BTC {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct BCH {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct ETH {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct LTC {
    pub available: String,
    pub orders: String,
}
#[derive(Debug, Deserialize)]
pub struct DASH {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct ZEC {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct USD {
    pub available: String,
    pub orders: String,
}
#[derive(Debug, Deserialize)]
pub struct EUR {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct GBP {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct RUB {
    pub available: String,
    pub orders: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceResult {
    pub timestamp: String,
    pub username: String,
    pub BTC: BTC,
    pub BCH: BCH,
    pub ETH: ETH,
    pub LTC: LTC,
    pub DASH: DASH,
    pub ZEC: ZEC,
    pub USD: USD,
    pub EUR: EUR,
    pub GBP: GBP,
    pub RUB: RUB,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrderResult {
    pub id: String,
    pub time: String,
    pub r#type: String,
    pub price: String,
    pub amount: String,
    pub pending: String,
    pub symbol1: String,
    pub symbol2: String,
}

#[derive(Deserialize, Debug)]
pub struct CancelOrdersByPairResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PlaceOrderResult {
    pub complete: bool,
    pub id: String,
    pub time: f64,
    pub pending: String,
    pub amount: String,
    pub r#type: String,
    pub price: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderResult {
    pub id: String,
    pub r#type: String,
    pub time: i64,
    pub lastTxTime: String,
    pub lastTx: String,
    //  pub pos: null,
    pub user: String,
    pub status: String,
    pub symbol1: String,
    pub symbol2: String,
    pub amount: String,
    pub price: String,
    // #[serde(rename = "fa:USD")]
    // pub fa_USD: String,
    // #[serde(rename = "ta:USD")]
    // pub ta_USD: String,
    pub remains: String,
    // #[serde(rename = "a:BTC:cds")]
    // pub a_BTC_cds: String,
    // #[serde(rename = "a:USD:cds")]
    // pub a_USD_cds: String,
    // #[serde(rename = "f:USD:cds")]
    // pub f_USD_cds: String,
    pub tradingFeeMaker: String,
    pub tradingFeeTaker: String,
    pub tradingFeeStrategy: String,
    pub orderId: String,
}

#[derive(Deserialize, Debug)]
pub struct PlaceOrderResult2 {
    pub symbol2Amount: String,
    pub symbol1Amount: String,
    pub time: i64,
    pub message: String,
    pub r#type: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxVtx {
    pub id: String,
    pub r#type: String,
    pub time: String,
    pub user: String,
    pub c: String,
    pub d: String,
    pub a: String,
    pub amount: String,
    pub balance: String,
    pub symbol: String,
    pub order: String,
    // pub buy: null,
    // pub sell: null,
    // pub pair: null,
    // pub pos: null,
    pub cs: String,
    // pub ds: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxData {
    pub id: String,
    pub r#type: String,
    pub time: i64,
    // lastTxTime: String, // TODO API different
    pub lastTxTime: i64,
    pub lastTx: String,
    pub user: String,
    pub status: String,
    pub symbol1: String,
    pub symbol2: String,
    pub amount: String,
    pub price: String,
    // #[serde(rename = "fa:USD")]
    // fa_USD: String,
    // #[serde(rename = "ta:USD")]
    // ta_USD: String,
    pub remains: String,
    // #[serde(rename = "a:BTC:cds")]
    // a_BTC_cds: String,
    #[serde(rename = "a:USD:cds")]
    pub a_USD_cds: String,
    // #[serde(rename = "f:USD:cds")]
    // f_USD_cds: String,
    pub tradingFeeMaker: String,
    pub tradingFeeTaker: String,
    pub tradingFeeStrategy: String,
    pub orderId: String,
    pub vtx: Vec<GetOrderTxVtx>,
    pub next: bool,
    // pub prev: bool,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxResult {
    pub e: String,
    pub ok: String,
    pub data: GetOrderTxData,
}

#[derive(Deserialize, Debug)]
pub struct ActiveOrderStatusResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<[String; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BTC_USD {
    pub buy: String,
    pub sell: String,
    pub buyMaker: String,
    pub sellMaker: String,
}

#[derive(Deserialize, Debug)]
pub struct DataMyfee {
    #[serde(rename = "BTC:USD")]
    pub BTC_USD: BTC_USD,
}

#[derive(Deserialize, Debug)]
pub struct GetMyfeeResult {
    pub e: String,
    pub ok: String,
    pub data: DataMyfee,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAddressResult {
    pub ok: String,
    pub e: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct LongShort {
    #[serde(rename = "2")]
    _2: Vec<String>,
    #[serde(rename = "3")]
    _3: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BTCUSDMarginalFee {
    short: LongShort,
    long: LongShort,
}

#[derive(Deserialize, Debug)]
struct DataMarginalFee {
    #[serde(rename = "BTC:USD")]
    BTC_USD: BTCUSDMarginalFee,
}

#[derive(Deserialize, Debug)]
pub struct GetMarginalFeeResult {
    e: String,
    ok: String,
    data: DataMarginalFee,
}

#[derive(Deserialize, Debug)]
pub struct TradeHistoryResult {
    pub r#type: String,
    pub date: String,
    pub amount: String,
    pub price: String,
    pub tid: String,
}

#[derive(Deserialize, Debug)]
pub struct OhlcvResult {
    pub time: i32,
    pub data1m: String,
    pub data1h: String,
    pub data1d: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookResult {
    pub timestamp: i64,
    pub bids: Vec<[f32; 2]>,
    pub asks: Vec<[f32; 2]>,
    pub pair: String,
    pub id: i64,
    pub sell_total: String,
    pub buy_total: String,
}

#[derive(Deserialize, Debug)]
pub struct PriceStatsResult {
    pub tmsp: i64,
    pub price: String,
}
