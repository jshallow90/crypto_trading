use serde::Deserialize;
use std::fs;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub ws : String,
    pub instrument_api: String,
    pub tickers: Vec<String>
}

impl Config {
    pub fn from_string(contents: &str) -> Self {
        let config: Self = serde_yaml::from_str(&contents)
            .expect("Could not load config!");
        config
    }
    
    pub fn from_file(file: &str) -> Self {
        let contents = fs::read_to_string(file)
            .expect("Could not open file!");
        let config: Self = Self::from_string(&contents);
        
        println!("{:?}", config); 
        config
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
    fn test_config() {
        let config = Config::from_string(CONFIG_STRING);
        
        assert_eq!(config.ws, "wss://test_websocket_url");
        assert_eq!(config.instrument_api, "https://api.binance.com/api/v3/exchangeInfo");
        assert_eq!(config.tickers, vec!["btcusdt@trade", "ethusdt@trade", "btcusdt@depth"]);
    }
}
