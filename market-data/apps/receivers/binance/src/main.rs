use receivers::binance::config::Config;
use receivers::binance::receiver::Receiver;


fn main() {

    /* TODO list
        - create process to listen and interpret books instead of trades
        - make the config command line option
        - create a hyperconfig listener
     */
    
    let config = Config::from_file("/home/jshallow/src/crypto-trading/market-data/apps/receivers/binance/src/config.yaml");
    let receiver = Receiver::from_config(&config);
    receiver.main();
}
