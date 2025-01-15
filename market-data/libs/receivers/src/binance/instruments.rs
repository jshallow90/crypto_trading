use std::collections::HashMap;

use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    timezone: String,
    #[serde(rename = "serverTime")]
    server_time: i64,
    #[serde(rename = "exchangeFilters")]
    exchange_filters: Vec<String>,
    symbols: Vec<SerializedInstrument>
}

#[derive(Debug)]
pub struct Instrument {
    pub symbol: String,
    pub base: String,
    pub quote: String
}

#[derive(Deserialize, Debug)]
pub struct SerializedInstrument {
    symbol: String,
    #[serde(rename = "baseAsset")]
    base_asset: String
}

impl SerializedInstrument {
    // symbol = BIOBNB
    // base_asset = BIO
    pub fn get_instrument(self) -> Instrument {
        let split: Vec<&str> = self.symbol.split(&self.base_asset).collect();
        let quote = split[1].to_owned();
        Instrument{
            symbol: self.symbol,
            base: self.base_asset,
            quote: quote
        }
    }
}

pub fn get_all_instruments(request_url: &str) -> Result<HashMap::<String, Instrument>, Error> {
    let client = reqwest::blocking::Client::new();
    let mut instrument_map: HashMap::<String, Instrument> = HashMap::new();
    
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client")
        .send()?;    
    
    let api_response: ApiResponse = response.json()?;
    for inst in api_response.symbols {
        instrument_map.insert(inst.symbol.to_owned(), inst.get_instrument());
    }

    println!("{:?}", instrument_map);
    
    Ok(instrument_map)
}