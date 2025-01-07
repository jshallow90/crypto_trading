use receivers::config::Config;
use receivers::receiver::Receiver;


fn main() {

    /* TODO list
        - track the last traded price (inside bid / ask)
        - create a hyperconfig listener
     */
    
    let config = Config::from_file("/home/jshallow/src/crypto-trading/market-data/apps/receivers/binance/src/config.yaml");
    let receiver = Receiver::from_config(&config);
    receiver.main();
}
