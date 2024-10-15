// src/utils/commands.rs
// author: Mia Stein

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(name = "coingator")]
#[clap(about = "🎲 a rusty stat arber toolkit")]
pub struct CliStruct {
    #[clap(subcommand)]
    pub command: CliEnum,
}


#[derive(Debug, Subcommand)]
pub enum CliEnum {
    #[clap(arg_required_else_help = true)]
    Dex(DexArgs),
    #[clap(arg_required_else_help = true)]
    Cex(CexArgs),
    #[clap(arg_required_else_help = true)]
    Whale(WhaleArgs),
    #[clap(arg_required_else_help = true)]
    Ev(EvArgs),
    #[clap(arg_required_else_help = true)]
    Bot(BotArgs),
}

#[derive(Debug, Args)]
pub struct DexArgs {
    #[clap(short, long)]
    pub tba: String,
}

#[derive(Debug, Args)]
pub struct CexArgs {
    #[clap(short, long)]
    pub tba: String,
}

#[derive(Debug, Args)]
pub struct WhaleArgs {
    #[clap(short, long)]
    pub tba: String,
}

#[derive(Debug, Args)]
pub struct EvArgs {
    #[clap(short, long)]
    pub tba: String,
}

#[derive(Debug, Args)]
pub struct BotArgs {
    #[clap(short, long)]
    pub tba: String,
}