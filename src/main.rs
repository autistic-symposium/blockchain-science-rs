// main.rs - author: steinkirch

use coingator::run;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    run().await;
    
}
