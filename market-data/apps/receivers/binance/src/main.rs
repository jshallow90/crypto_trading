use receivers::binance::config::Config;
use receivers::binance::receiver::Receiver;


fn main() {

    /* TODO list
        - track the last traded price (inside bid / ask)
        - make the config command line option
        - create a hyperconfig listener
        - create process to listen and interpret books
     */
    
    let config = Config::from_file("/home/jshallow/src/crypto-trading/market-data/apps/receivers/binance/src/config.yaml");
    let receiver = Receiver::from_config(&config);
    receiver.main();
}
