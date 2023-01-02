// lib.rs - author: steinkirch


use std::env;

pub mod markets;
use crate::markets::bbit;


pub async fn run() {
    
    println!("\n🐊 welcome to coingator 🪙. type your option:\n");
    println!("➡ 1: sub to public topics for a derivative (e.g., ETHUSDT)");
    println!("➡ 2: sub to public topics for a pair of derivatives");
    println!("➡ 3: sub to public inverse perpetual info topics");
    println!("➡ 4: sub to spot local order book topics");
    println!("➡ 5: sub to private inverse execution topics");
    println!("➡ 6: sub to private positions topics\n");


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
            "5" => bbit::subscribe_exec().await,
            "6" => bbit::subscribe_positions().await,
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

