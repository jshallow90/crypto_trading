use crate::binance::config::Config;
use crate::binance::instruments::{Instrument, get_all_instruments};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use serde_json;
use serde_this_or_that::as_f64;
use tungstenite::{connect, Message};


#[derive(Debug, Deserialize)]
struct PriceQty {
    #[serde(deserialize_with = "as_f64")]
    price: f64,
    #[serde(deserialize_with = "as_f64")]
    qty: f64
}

#[derive(Debug, Deserialize)]
struct BookUpdate {
    #[serde(rename = "lastUpdateId")]
    last_update_id: i64,
    bids: Vec<PriceQty>,
    asks: Vec<PriceQty>
}

#[derive(Debug, Deserialize)]
struct BookStream {
    data: BookUpdate,
    stream: String,
}


#[derive(Debug)]
struct PriceMap {
    // base: &str,
    // quote: &str,
    bid: f64,
    ask: f64,
    wmp: f64,
    update_time: Duration
}

impl PriceMap {
    // fn new(base: &str, quote: &str, bid: f64, ask: f64, wmp: f64, update_time: Duration) -> Self {
    fn new(bid: f64, ask: f64, wmp: f64, update_time: Duration) -> Self {
        Self {
            // base,
            // quote,
            bid,
            ask,
            wmp,
            update_time
        }
    }
}


#[derive(Debug)]
pub struct Receiver {
    ws: String,
    tickers: Vec<String>,
    instrument_map: HashMap<String, Instrument>,
    price_map: HashMap<String, PriceMap>
}

impl Receiver {
    pub fn from_config(config: &Config) -> Self {
        Self {
            ws: config.ws.to_owned(),
            tickers: config.tickers.to_owned(),
            instrument_map: get_all_instruments(&config.instrument_api).expect("Cannot get instrument data!"),
            price_map: HashMap::new()
        }
    }

    pub fn new(ws: String, tickers: Vec<String>, instrument_map: HashMap<String, Instrument>) -> Self {
        Self {
            ws,
            tickers,
            instrument_map,
            price_map: HashMap::new()
        }
    }

    pub fn get_api_url(&self) -> String {
        let tickers = self.tickers.join("/");
        format!("{0}/stream?streams={1}&timeUnit=MICROSECOND", self.ws, tickers)
    }
    
    pub fn on_event(&mut self, msg: Message) {
        let text = match msg.to_text() {
            Ok(v) => v,
            Err(_) => {
                println!("Could not parse Message {msg}");
                return
            }
        };
        
        let parsed: BookStream = match serde_json::from_str(&text) {
            Ok(s) => s,
            Err(e) => { 
                println!("Could not parse JSON for {:?}, {:?}", msg, e);
                return
             }
        };
        
        // TODD make this much nicer
        let ticker: String = (&parsed.stream.split("@").collect::<Vec<_>>()[0]).to_string().to_uppercase();
        let pair: &Option<&Instrument> = &self.instrument_map.get(&ticker);
        let ccy_pair = match pair {
            Some(t) => format!("{}/{}", t.base, t.quote),
            None => ticker.clone()
        };

        let inside_bid: &PriceQty = &parsed.data.bids[0];
        let inside_ask: &PriceQty = &parsed.data.asks[0];
        let mwp: f64 = (inside_bid.price * inside_ask.qty + inside_ask.price * inside_bid.qty) / (inside_bid.qty + inside_ask.qty);
        
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let price_update: PriceMap = PriceMap::new(inside_bid.price, inside_ask.price, mwp, ts);
        let _ = &self.price_map.insert(ticker, price_update);
        println!("MAP={:?}", &self.price_map);
    }

    pub fn main(mut self) {
        let api_url = &self.get_api_url();
    
        let (mut socket, response) = connect(api_url).expect("Can't connect");
        println!("Connecting to the server");
        println!("Response HTTP code: {}", response.status());

        loop {
            let msg = socket.read().expect("Error reading message");
            let _ = &self.on_event(msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG_STRING: &str = "
    ws: \"wss://test_websocket_url\"
    instrument_api: \"https://api.binance.com/api/v3/exchangeInfo\"
    tickers:
      - \"btcusdt@trade\"
      - \"ethusdt@trade\"
      - \"btcusdt@depth\"
    ";

    #[test]
    fn test_receiver() {
        let config = Config::from_string(CONFIG_STRING);
        let receiver = Receiver::from_config(&config);

        assert_eq!(receiver.ws, "wss://test_websocket_url");
        assert_eq!(receiver.tickers, vec!["btcusdt@trade", "ethusdt@trade", "btcusdt@depth"]);
        assert_eq!(receiver.get_api_url(), "wss://test_websocket_url/stream?streams=btcusdt@trade/ethusdt@trade/btcusdt@depth&timeUnit=MICROSECOND");
    }
}
