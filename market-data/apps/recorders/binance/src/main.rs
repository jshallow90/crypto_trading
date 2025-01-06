use recorders::Config;
use tungstenite::connect;


fn main() {

    /* TODO list
        - create a struct for the websocket reader
        - start up listener for multiple symbols
        - track the last traded price (inside bid / ask)
     */
    
    let config = Config::from_file("/home/jshallow/src/crypto-trading/market-data/apps/recorders/binance/src/config.yaml");
    let api_url = config.get_api_url();
    
    let (mut socket, response) = connect(api_url).expect("Can't connect");
    println!("Connecting to the server");
    println!("Response HTTP code: {}", response.status());

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");
    }

}
