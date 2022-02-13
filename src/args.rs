use clap::{Parser, Subcommand};

// clap Parser struct
#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

// Command enum for use by Args
#[derive(Subcommand)]
pub enum Command {
    Push,
    Revert { version: Option<i32> },
}
