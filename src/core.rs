use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Deserialize, Serialize)]
pub struct MarketData {
    pub current_price: HashMap<String, f32>,
    ath: HashMap<String, f32>,
    market_cap: HashMap<String, f32>,
    market_cap_rank: usize,
    pub high_24h: HashMap<String, f32>,
    low_24h: HashMap<String, f32>,
    price_change_24h: f32,
    pub price_change_percentage_24h_in_currency: HashMap<String, f32>,
}
#[derive(Deserialize, Serialize)]
pub struct CoinData {
    id: String,
    pub symbol: String,
    pub name: String,
    image: HashMap<String, String>,
    market_cap_rank: usize,
    pub market_data: MarketData,
}

#[derive(Deserialize, Serialize)]
pub struct CoinKeyNameData {
    id: String,
    symbol: String,
    name: String,
}

pub async fn get_data<T>(url: &Url) -> Result<T, JsValue>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("{}", url);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;

    match resp_value {
        Ok(rv) => {
            let resp: Response = rv.dyn_into().unwrap();
            let json = JsFuture::from(resp.json()?).await?;
            let info = json.into_serde().unwrap();
            Ok(info)
        }
        Err(e) => Err(e),
    }
}
