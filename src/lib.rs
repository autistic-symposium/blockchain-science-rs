// lib.rs - author: steinkirch


use std::env;

pub mod trade;
pub mod markets;

use crate::markets::bbit;
use crate::markets::bmex;
use crate::markets::bnb;
use crate::trade::bbitbot;
use crate::trade::bnbbot;
use crate::trade::bmexbot;


pub async fn run() {
    
    println!("\n🐊 welcome to coingator 🪙. type your option:\n");
    println!("➡ 1: sub to public topics for a derivative");
    println!("➡ 2: sub to public topics for a pair of derivatives");
    println!("➡ 3: sub to public topics for inverse perpetual contracts");
    println!("➡ 4: sub to public topics for spot local orderbook");
    println!("➡ 5: get cointegration for two symbols");
    println!("➡ 6: deploy coingator bot\n");

    // create an argument input
    let mut input = String::new();

    // read the input
    std::io::stdin().read_line(&mut input).unwrap();

    // create a vector of arguments
    let mut args = input.split_whitespace();

    // get the command
    let command = args.next().unwrap();

    // select which cex to use
    let cex = &env::var("CEX").expect("⛔️ CEX must be set on .env file");
    
    if cex == "bybit" {
        match command {
            "1" => bbit::subscribe_coin().await,
            "2" => bbit::subscribe_pairs().await,
            "3" => bbit::subscribe_perpetual().await,
            "4" => bbit::subscribe_spot().await,
            "5" => bbitbot::find_cointegration().await,
            "6" => bbitbot::run_bot().await,
            _ => println!("command not found: {}", command),
        }
    
    } else if cex == "binance" {
        match command {
            "1" => bnb::subscribe_coin().await,
            "2" => bnb::subscribe_pairs().await,
            "3" => bnb::subscribe_perpetual().await,
            "4" => bnb::subscribe_spot().await,
            "5" => bnbbot::find_cointegration().await,
            "6" => bnbbot::run_bot().await,
            _ => println!("command not found: {}", command),
        }
    
    } else if cex == "bitmex" {
        match command {
            "1" => bmex::subscribe_coin().await,
            "2" => bmex::subscribe_pairs().await,
            "3" => bmex::subscribe_perpetual().await,
            "4" => bmex::subscribe_spot().await,
            "5" => bmexbot::find_cointegration().await,
            "6" => bmexbot::run_bot().await,
            _ => println!("command not found: {}", command),
        }

    } else {
        println!("⛔️ {} is not a valid CEX", cex);
    }

}

