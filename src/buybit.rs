// buybit.rs - author: steinkirch

use std::env;
use bybit::spot::ws::{PublicResponse, PublicWebSocketApiClient};


pub async fn subscribe() {

    let coin_pair = &env::var("COIN_PAIR").expect("COIN_PAIR must be set on env file");
    let mut client = PublicWebSocketApiClient::new();

    client.subscribe_trade(coin_pair, false);
    client.subscribe_realtimes(coin_pair, false);
    client.subscribe_kline(coin_pair, "1m", false);
    client.subscribe_depth(coin_pair, false);
    client.subscribe_merged_depth(coin_pair, false, 1);
    client.subscribe_diff_depth(coin_pair, false);


    let callback = |res: PublicResponse| match res {
        PublicResponse::Trade(res) => println!("Trade: {:?}", res),
        PublicResponse::Realtimes(res) => println!("Realtimes: {:?}", res),
        PublicResponse::Kline(res) => println!("Kline: {:?}", res),
        PublicResponse::Depth(res) => println!("Depth: {:?}", res),
        PublicResponse::MergedDepth(res) => println!("Merged depth: {:?}", res),
        PublicResponse::DiffDepth(res) => println!("Diff depth: {:?}", res),
        PublicResponse::LT(res) => println!("LT: {:?}", res),
        PublicResponse::Pong(res) => println!("Pong: {:?}", res),
        PublicResponse::Ping(res) => println!("Ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn coin() {
    println!("retrieve data for a currency");
}


pub async fn history() {
    println!("retrieve price history for currency and time period");
}
