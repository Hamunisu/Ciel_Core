// update: 2026/05/03

use serde::Deserialize;
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

use crate::module::{c_core, escape, input_check};
mod module;

#[derive(Deserialize, Debug)]
struct IcaoData {
    input: &'static str,
    icao: &'static str,
}

#[derive(Deserialize, Debug)]
struct FlightsData {
    dep: String,
    arr: String,
    num: Vec<u16>,
}

static ICAO_BYTES: &[u8] = include_bytes!("../postcard/icao.ciel");
static ICAO_DATA: OnceLock<Vec<IcaoData>> = OnceLock::new();

static FLIGHTS_BYTES: &[u8] = include_bytes!("../postcard/number.ciel");
static FLIGHTS_DATA: OnceLock<Vec<FlightsData>> = OnceLock::new();

#[wasm_bindgen]
pub fn search(deperture: &str, arrival: &str) -> String {
    // 入力確認処理
    let deperture = escape(deperture).to_uppercase();
    let arrival = escape(arrival).to_uppercase();

    let (chk_dep, chk_arr) = match input_check(deperture.trim(), arrival.trim()) {
        Ok((d, a)) => (d, a),
        Err(e) => return e,
    };

    c_core(&chk_dep, &chk_arr)
}
