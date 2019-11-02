use std::collections::HashMap;

use crate::models::Symbol;

use serde::Deserialize;

const ROOT: &str = "https://cex.io/api/";

pub(crate) fn make_get_request<T>(url: &str) -> Result<T, reqwest::Error>
where
    for<'de> T: Deserialize<'de>,
{
    get_req(url).unwrap().json()
}

pub(crate) fn make_get_request_symbols<T>(
    base_url: &str,
    symbol1: Symbol,
    symbol2: Symbol,
) -> Result<T, reqwest::Error>
where
    for<'de> T: Deserialize<'de>,
{
    let url = format!("{}{}/{}", base_url, symbol1, symbol2);
    get_req(&url).unwrap().json()
}

pub(crate) fn get_req(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let full_url = format!("{}{}", ROOT, url);
    reqwest::get(&full_url)
}

#[allow(dead_code)]
pub(crate) fn make_get_request_text(url: &str) -> Result<String, reqwest::Error> {
    get_req(&url).unwrap().text()
}

pub(crate) fn make_post_request<T>(
    signature: HashMap<String, String>,
    base_url: &str,
    mut params: HashMap<String, String>,
) -> Result<T, reqwest::Error>
where
    for<'de> T: Deserialize<'de>,
{
    post_req(signature, base_url, &mut params).unwrap().json()
}

#[allow(dead_code)]
pub(crate) fn make_post_request_text(
    signature: HashMap<String, String>,
    base_url: &str,
    mut params: HashMap<String, String>,
) -> Result<String, reqwest::Error> {
    post_req(signature, base_url, &mut params).unwrap().text()
}

pub(crate) fn make_post_request_symbols<T>(
    signature: HashMap<String, String>,
    base_url: &str,
    symbol1: Symbol,
    symbol2: Symbol,
    mut params: HashMap<String, String>,
) -> Result<T, reqwest::Error>
where
    for<'de> T: Deserialize<'de>,
{
    let url = format!("{}{}/{}", base_url, symbol1, symbol2);
    post_req(signature, &url, &mut params).unwrap().json()
}

fn post_req(
    signature: HashMap<String, String>,
    url: &str,
    params: &mut HashMap<String, String>,
) -> Result<reqwest::Response, reqwest::Error> {
    for k in signature.keys() {
        params.insert(k.to_string(), signature[k].clone());
    }
    let client = reqwest::Client::new();
    client.post(&format!("{}{}", ROOT, url)).json(params).send()
}
