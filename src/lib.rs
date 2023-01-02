// lib.rs - author: steinkirch


use std::env;

pub mod markets;
use crate::markets::bbit;


pub async fn run() {
    
    println!("\n🐊 welcome to coingator 🪙. type your option:\n");
    println!("➡ coin: subscribe to public topics for a coin (eg. ETHUSDT)");
    println!("➡ pairs: subscribe to public topics for a pair (e.g. BTCUSDT, ETHUSDT)");
    println!("➡ exec: subscribe to private topics (e.g. execution)");
    println!("➡ positions: subscribe to private positions topics \n");

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
            "coin" => bbit::subscribe_coin().await,
            "pairs" => bbit::subscribe_pairs().await,
            "exec" => bbit::subscribe_exec().await,
            "positions" => bbit::subscribe_positions().await,
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

