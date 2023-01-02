// lib.rs - author: steinkirch

mod buybit;


pub async fn run() {
    
    println!("\n🏭 welcome to cointbot 🪙. type your option:\n");
    println!("➡ subscribe: subscribe to a topic");
    println!("➡ coin: get data for a currency");
    println!("➡ history: get price history for currency and time period\n");

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
        "subscribe" => buybit::subscribe().await,
        "topics" => buybit::coin().await,
        "history" => buybit::history().await,
        _ => println!("command not found: {}", command),
    }
    
}

