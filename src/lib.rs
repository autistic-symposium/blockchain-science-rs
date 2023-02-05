// lib.rs - author: steinkirch


use std::env;

pub mod markets;
pub mod trade;
use crate::markets::bbit;
use crate::trade::bot1;


pub async fn run() {
    
    println!("\n🐊 welcome to coingator 🪙. type your option:\n");
    println!("➡ 1: sub to public topics for a derivative");
    println!("➡ 2: sub to public topics for a pair of derivatives");
    println!("➡ 3: sub to public inverse perpetual info topics");
    println!("➡ 4: sub to spot local orderbook topics");
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
            "5" => bot1::find_cointegration().await,
            "6" => bot1::run_bot1().await,
            _ => println!("command not found: {}", command),
        }
    
    } else if cex == "binance" {
        println!("⛔️ {} is not supported yet", cex);
    
    } else if cex == "bitmex" {
        println!("⛔️ {} is not supported yet", cex);

    } else {
        println!("⛔️ {} is not a valid CEX", cex);
    }

}

