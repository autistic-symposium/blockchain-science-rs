// lib.rs - author: steinkirch


use std::env;

pub mod txs;
pub mod cexs;
pub mod dexs;
pub mod coingecko;
pub mod searchers;


use crate::cexs::bbit;
use crate::cexs::bmex;
use crate::cexs::bnb;
use crate::dexs::mempool;
use crate::txs::decoder;
use crate::market::coingecko;
use crate::searchers::searcher_one;




pub async fn run() {
    
    println!("\n🐊 welcome to coingator 🪙. type your option:\n");
    println!("➡ 1: sub to public topics for a derivative");
    println!("➡ 2: sub to public topics for a pair of derivatives");
    println!("➡ 3: sub to public topics for inverse perpetual contracts");
    println!("➡ 4: sub to public topics for spot local orderbook");
    println!("➡ 5: sub to the public mempool topic");
    println!("➡ 6: test searcher boilerplate");
    println!("➡ 7: run calldata decoder");
    println!("➡ 8: run tx decoder");

    // create an argument input
    let mut input = String::new();

    // read the input
    std::io::stdin().read_line(&mut input).unwrap();

    // create a vector of arguments
    let mut args = input.split_whitespace();

    // get the command
    let command = args.next().unwrap();

    // select which cex to use
    let source = &env::var("SOURCE").expect("⛔️ SOURCE must be set on .env file");

    if source == "bybit" {
        match command {
            "1" => bbit::subscribe_coin().await,
            "2" => bbit::subscribe_pairs().await,
            "3" => bbit::subscribe_perpetual().await,
            "4" => bbit::subscribe_spot().await,
            "5" => mempool::run().await,
            "6" => searcher_one::run().await,
            "7" => decoder::run().await,
            _ => println!("command not found: {}", command),
        }

    } else if source == "coingecko" {
        match command {
            "1" => coingecko::subscribe_coin().await,
            "2" => coingecko::subscribe_pairs().await,
            "3" => coingecko::subscribe_perpetual().await,
            "4" => coingecko::subscribe_spot().await,
            "5" => mempool::run().await,
            "6" => searcher_one::run().await,
            "7" => decoder::run().await,
            _ => println!("command not found: {}", command),
        }
        

    } else if source == "binance" {
        match command {
            "1" => bnb::subscribe_coin().await,
            "2" => bnb::subscribe_pairs().await,
            "3" => bnb::subscribe_perpetual().await,
            "4" => bnb::subscribe_spot().await,
            "5" => mempool::run().await,
            "6" => searcher_one::run().await,
            "7" => decoder::run().await,
            _ => println!("command not found: {}", command),
        }
    
    } else if source == "bitmex" {
        match command {
            "1" => bmex::subscribe_coin().await,
            "2" => bmex::subscribe_pairs().await,
            "3" => bmex::subscribe_perpetual().await,
            "5" => mempool::run().await,
            "6" => searcher_one::run().await,
            "7" => decoder::run().await,
            _ => println!("command not found: {}", command),
        }

    } else {
        match command {
            "5" => mempool::run().await,
            "6" => searcher_one::run().await,
            "7" => decoder::run().await,
            _ => println!("command not found: {}", command),
        }
    }

}

