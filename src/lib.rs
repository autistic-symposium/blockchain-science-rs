// lib.rs - author: steinkirch

use std::env;

mod buybit;


pub async fn run() {
    
    println!("🏭 welcome to cointbot 🪙. type your option: ");
    println!("coin: get data for a currency (coin)");
    println!("history: get price history for currency (coin) and time period (time)");

    // create an argument input
    let mut input = String::new();

    // read the input
    std::io::stdin().read_line(&mut input).unwrap();

    // create a vector of arguments
    let mut args = input.split_whitespace();

    // get the command
    let command = args.next().unwrap();

    // match the command
    match command {
        "coin" => buybit::coin().await,
        "history" => buybit::history().await,
        _ => println!("command not found: {}", command),
    }
    
}

