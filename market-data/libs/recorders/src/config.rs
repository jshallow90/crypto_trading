use crate::test_consts;
use serde::Deserialize;
use std::fs;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub ws : String,
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

    pub fn get_api_url(self) -> String {
        let tickers = self.tickers.join("/");
        format!("{0}/stream?streams={1}&timeUnit=MICROSECOND", self.ws, tickers)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_string(test_consts::CONFIG_STRING);
        
        assert_eq!(config.ws, "wss://test_websocket_url");
        assert_eq!(config.tickers, vec!["btcusdt@trade", "ethusdt@trade", "btcusdt@depth"]);
        assert_eq!(config.get_api_url(), "wss://test_websocket_url/stream?streams=btcusdt@trade/ethusdt@trade/btcusdt@depth&timeUnit=MICROSECOND");
    }
}
