// src/utils/cli_handler.rs
// author: Mia Stein

use std::env;

use crate::{
    utils::commands::{DexArgs,
                      CexArgs,
                      WhaleArgs,
                      EvArgs,
                      BotArgs},
    market::coingecko::{get_coin_info},
};

/////////////////////////////////////////
// Public functions
/////////////////////////////////////////


pub async fn handle_dexs(args: DexArgs) {

    println!("to be implemented");
}

pub async fn handle_cexs(args: CexArgs) {

    println!("to be implemented");
}

pub async fn handle_market(args: WhaleArgs) {

    println!("to be implemented");
}

pub async fn handle_science(args: EvArgs) {

    println!("to be implemented");
}

pub async fn handle_searcher(args: BotArgs) {

    println!("to be implemented");
}
