// main.rs - author: steinkirch

use coingator::run;


#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("failed to read .env file");
    run().await;
    
}
