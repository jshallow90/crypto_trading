use crate::config::Config;
use crate::test_consts;

use tungstenite::connect;

pub struct Receiver {
    config: Config
}

impl Receiver {
    pub fn from_string(contents: &str) -> Self {
        let config = Config::from_string(contents);
        Self {
            config
        }
    }
    
    pub fn from_file(file: &str) -> Self {
        let config = Config::from_file(file);
        Self {
            config
        }
    }

    pub fn main(self) {
        let api_url = self.config.get_api_url();
    
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

    #[test]
    fn test_receiver() {
        let receiver = Receiver::from_string(test_consts::CONFIG_STRING);

        assert_eq!(receiver.config.ws, "wss://test_websocket_url");
        assert_eq!(receiver.config.tickers, vec!["btcusdt@trade", "ethusdt@trade", "btcusdt@depth"]);
        assert_eq!(receiver.config.get_api_url(), "wss://test_websocket_url/stream?streams=btcusdt@trade/ethusdt@trade/btcusdt@depth&timeUnit=MICROSECOND");
    }
}
