use crate::binance::config::Config;
use std::collections::HashMap;
use serde::Deserialize;

use serde_json;
use tungstenite::{connect, Message};

#[derive(Debug, Deserialize)]
struct Trade {
    E: i64,
    M: bool,
    T: i64,
    e: String,
    m: bool,
    p: String,
    q: String,
    s: String,
    t: i64,
}

#[derive(Debug, Deserialize)]
struct TradeStream {
    data: Trade,
    stream: String,
}

#[derive(Debug)]
struct PriceMap {
    price: f64,
    update_time: i64
}

impl PriceMap {
    fn new(price: f64, update_time: i64) -> Self {
        Self {
            price,
            update_time
        }
    }
}


#[derive(Debug)]
pub struct Receiver {
    ws: String,
    tickers: Vec<String>,
    price_map: HashMap<String, PriceMap>
}

impl Receiver {
    pub fn from_config(config: &Config) -> Self {
        Self {
            ws: config.ws.to_owned(),
            tickers: config.tickers.to_owned(),
            price_map: HashMap::new()
        }
    }

    pub fn new(ws: String, tickers: Vec<String>) -> Self {
        Self {
            ws,
            tickers,
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
        
        let parsed: TradeStream = match serde_json::from_str(&text) {
            Ok(s) => s,
            Err(e) => { 
                println!("Could not parse JSON for {:?}, {:?}", msg, e);
                return
             }
        };
        
        println!("Price={:?}, T={:?}, q={:?}, s={:?}, \nparsed={:?}\n\n", parsed.data.p, parsed.data.T, parsed.data.q, parsed.data.s, parsed);
        let price: f64 = parsed.data.p.parse().unwrap();
        let price_update: PriceMap = PriceMap::new(price, parsed.data.T);
        &self.price_map.insert(parsed.data.s, price_update);
        // println!("MAP={:?}", &self.price_map);
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
