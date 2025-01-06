use std::fs;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    ws : String,
    tickers: Vec<String>
}

impl Config {
    fn read_config(contents: &str) -> Self {
        let config: Self = serde_yaml::from_str(&contents)
            .expect("Could not load config!");
        config
    }
    
    pub fn from_file(file: &str) -> Self {
        let contents = fs::read_to_string(file)
            .expect("Could not open file!");
        let config: Self = Self::read_config(&contents);
        
        println!("{:?}", config); 
        config
    }

    pub fn get_api_url(self) -> String {
        let tickers = self.tickers.join("/");
        format!("{0}/stream?streams={1}&timeUnit=MICROSECOND", self.ws, tickers)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_attributes() {
        let config_string = "
ws: \"wss://test_websocket_url\"
tickers:
  - \"btcusdt@trade\"
  - \"ethusdt@trade\"
  - \"btcusdt@depth\"
";
        let config = Config::read_config(config_string);
        assert_eq!(config.ws, "wss://test_websocket_url");
        assert_eq!(config.tickers, vec!["btcusdt@trade", "ethusdt@trade", "btcusdt@depth"]);
        assert_eq!(config.get_api_url(), "wss://test_websocket_url/stream?streams=btcusdt@trade/ethusdt@trade/btcusdt@depth&timeUnit=MICROSECOND");
    }
}
