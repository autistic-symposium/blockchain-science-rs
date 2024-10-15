// main.rs
// author: Mia Stein

mod utils;
mod cexs;
mod dexs;
mod market;
mod science;
mod searcher;

use clap::Parser;

use utils::{CliEnum, 
            CliStruct,
            handle_cexs,
            handle_dexs,
            handle_market,
            handle_science,
            handle_searcher};

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    let args = CliStruct::parse();
    
    match args.command {
        CliEnum::Dex(args) => handle_dexs(args).await,
        CliEnum::Cex(args) => handle_cexs(args).await,
        CliEnum::Whale(args) => handle_market(args).await,
        CliEnum::Ev(args) => handle_science(args).await,
        CliEnum::Bot(args) => handle_searcher(args).await,
    }
}