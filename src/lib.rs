use wasm_bindgen::prelude::*;
pub mod core;
pub mod utils;
use crate::core::{CoinData, CoinKeyNameData};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

const BASE_URL: &str = "https://api.coingecko.com/api/v3/coins/";

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &JsValue);
}

#[derive(Deserialize, Serialize)]
struct DeserializableData<T> {
    data: T,
}

#[wasm_bindgen]
pub async fn get_coins_list() -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let coins_list_data = core::get_data::<Vec<CoinKeyNameData>>(
        &utils::parse_url(&format!("{}/{}", BASE_URL, "list")).unwrap(),
    )
    .await;

    if let Ok(gg) = coins_list_data {
        let result = DeserializableData { data: gg };
        return Ok(JsValue::from_serde(&result).unwrap());
    }

    return Ok(JsValue::null());
}

#[wasm_bindgen]
pub async fn get_coins_data(tkns: String) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let mut tokens = [
        "bitcoin".to_owned(),
        "ethereum".to_owned(),
        "litecoin".to_owned(),
    ]
    .to_vec();
    for split in tkns.split(",") {
        if split != "" {
            tokens.push(split.to_owned());
        }
    }

    let mut urls = Vec::new();

    for token in tokens {
        urls.push(utils::parse_url(&format!("{}{}", BASE_URL, token)).unwrap());
    }

    let mut coins_data = Vec::new();
    for iurl in urls {
        let data = core::get_data::<CoinData>(&iurl).await;
        match data {
            Ok(cd) => {
                coins_data.push(cd);
            }
            Err(e) => {}
        }
    }

    let result = DeserializableData {
        data: coins_data,
    };

    return Ok(JsValue::from_serde(&result).unwrap());
}
