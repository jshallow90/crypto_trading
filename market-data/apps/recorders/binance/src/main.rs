use recorders::receiver::Receiver;


fn main() {

    /* TODO list
        - change the receiver args to websocket url and tickers
        - track the last traded price (inside bid / ask)
        - create a hyperconfig listener
     */
    
    let receiver = Receiver::from_file("/home/jshallow/src/crypto-trading/market-data/apps/recorders/binance/src/config.yaml");
    receiver.main();
}
