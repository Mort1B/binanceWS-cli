use tungstenite::connect;
use url::Url;

mod models;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

fn main() {
    //top 5 bids and asks for EHT_BTC every 100 ms
    let binance_url = format!(
        "{}/stream?streams=ethbtc@depth5@100ms/bnbeth@depth5@100ms",
        BINANCE_WS_API
    );
    let (mut socket, response) =
        connect(Url::parse(&binance_url).unwrap()).expect("Cannot connect to ");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => panic!("Error getting text"),
        };

        let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Cant parse");
        for i in 0..parsed.data.asks.len() {
            println!(
                "{}: {}. ask: {}, size: {}",
                parsed.stream, i, parsed.data.asks[i].price, parsed.data.asks[i].size
            )
        }
    }
}
