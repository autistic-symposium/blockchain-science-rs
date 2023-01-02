// buybit.rs - author: steinkirch

use std::env;
use bybit::linear::{PublicResponse, PublicWebSocketApiClient};
use bybit::inverse::{PrivateResponse, PrivateWebSocketApiClient};
use bybit::spot::ws::{PublicV2Response, PublicV2WebSocketApiClient, PrivateResponse as OtherPrivateResponse, PrivateWebSocketApiClient as OtherPrivateWebSocketApiClient};


pub async fn subscribe_coin() {

    let coin = &env::var("COIN").expect("⛔️ COIN must be set on .env file");
    let mut client = PublicV2WebSocketApiClient::new();
    println!("🐊 subcribing to websockets for: {:?} \n", coin);

    client.subscribe_depth(coin, false);
    client.subscribe_trade(coin, false);
    client.subscribe_book_ticker(coin, false);
    client.subscribe_realtimes(coin, false);

    let callback = |res: PublicV2Response| match res {
        PublicV2Response::Depth(res) => println!("✅ depth: {:?}", res),
        PublicV2Response::Kline(res) => println!("✅ kline: {:?}", res),
        PublicV2Response::Trade(res) => println!("✅ trade: {:?}", res),
        PublicV2Response::BookTicker(res) => println!("✅ book ticker: {:?}", res),
        PublicV2Response::Realtimes(res) => println!("✅ realtimes: {:?}", res),
        PublicV2Response::Pong(res) => println!("✅ pong: {:?}", res),
        PublicV2Response::Ping(res) => println!("✅ ping: {:?}", res),
    };


    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn subscribe_pairs() {
    
    let pairs = &env::var("PAIRS").expect("⛔️ PAIRS must be set on .env file");
    let symbols: Vec<&str> = pairs.split(",").collect();
    println!("🐊 subcribing to websockets for: {:?} \n", symbols);

    let mut client = PublicWebSocketApiClient::new();

    client.subscribe_order_book_l2_25(&symbols);
    client.subscribe_order_book_l2_200(&symbols);
    client.subscribe_trade(&symbols);
    client.subscribe_instrument_info(&symbols);
    client.subscribe_kline(&symbols, "1");
    client.subscribe_liquidation(&symbols);

    let callback = |res: PublicResponse| match res {
        PublicResponse::OrderBookL2Snapshot(res) => println!("✅ order book L2 snapshot: {:?}", res),
        PublicResponse::OrderBookL2Delta(res) => println!("✅ order book L2 delta: {:?}", res),
        PublicResponse::Trade(res) => println!("✅ trade: {:?}", res),
        PublicResponse::InstrumentInfoSnapshot(res) => {
            println!("✅ instrument info snapshot: {:?}", res)
        }
        PublicResponse::InstrumentInfoDelta(res) => {
            println!("✅ instrument info delta: {:?}", res)
        }
        PublicResponse::Kline(res) => println!("✅ kline: {:?}", res),
        PublicResponse::Liquidation(res) => println!("✅ liquidation: {:?}", res),
    };


    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),    
    }
}


pub async fn subscribe_exec() {
    
    let api_key = &env::var("BYBIT_API_KEY").expect("⛔️ BYBIT_API_KEY must be set on .env file");
    let api_secret = &env::var("BYBIT_API_SECRET").expect("⛔️ BYBIT_API_SECRET must be set on .env file");

    println!("🐊 subcribing to private websockets: \n");

    let client = OtherPrivateWebSocketApiClient::builder().testnet()
                                .build_with_credentials(&api_key, &api_secret);

    let callback = |res: OtherPrivateResponse| match res {
        OtherPrivateResponse::ExecutionReportSequence(seq) => println!("✅ execution report: {:?}", seq),
        OtherPrivateResponse::TicketInfoSequence(seq) => println!("✅ ticket info: {:?}", seq),
        OtherPrivateResponse::OutboundAccountInfoSequence(seq) => {
            println!("✅ outbound account info: {:?}", seq)
        },
        OtherPrivateResponse::Pong(res) => println!("✅ pong: {:?}", res),
        OtherPrivateResponse::Ping(res) => println!("✅ ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn subscribe_positions() {
    
    let api_key = &env::var("BYBIT_API_KEY").expect("⛔️ BYBIT_API_KEY must be set on .env file");
    let api_secret = &env::var("BYBIT_API_SECRET").expect("⛔️ BYBIT_API_SECRET must be set on .env file");

    println!("🐊 subcribing to private positions websockets: \n");

    let mut client = PrivateWebSocketApiClient::new(api_key, api_secret);

    client.subscribe_position();
    client.subscribe_execution();
    client.subscribe_order();
    client.subscribe_stop_order();
    client.subscribe_wallet();

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::Position(res) => println!("✅ position: {:?}", res),
        PrivateResponse::Execution(res) => println!("✅ execution: {:?}", res),
        PrivateResponse::Order(res) => println!("✅ order: {:?}", res),
        PrivateResponse::StopOrder(res) => println!("✅ stop order: {:?}", res),
        PrivateResponse::Wallet(res) => println!("✅ wallet: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}