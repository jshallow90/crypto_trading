use crate::config::Config;

use tungstenite::connect;

pub struct Receiver {
    ws: String,
    tickers: Vec<String>
}

impl Receiver {
    pub fn from_config(config: &Config) -> Self {
        Self {
            ws: config.ws.to_owned(),
            tickers: config.tickers.to_owned()
        }
    }

    pub fn new(ws: String, tickers: Vec<String>) -> Self {
        Self {
            ws,
            tickers
        }
    }

    pub fn get_api_url(self) -> String {
        let tickers = self.tickers.join("/");
        format!("{0}/stream?streams={1}&timeUnit=MICROSECOND", self.ws, tickers)
    }
    
    pub fn main(self) {
        let api_url = self.get_api_url();
    
        let (mut socket, response) = connect(api_url).expect("Can't connect");
        println!("Connecting to the server");
        println!("Response HTTP code: {}", response.status());

        loop {
            let msg = socket.read().expect("Error reading message");
            println!("Received: {msg}");
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
