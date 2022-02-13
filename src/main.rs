// External crates
use clap::Parser;

// Internal crates
use rvc::args::*;

fn main() {
    // Obtain arguments
    let args = Args::parse();

    // Separate by command
    match args.command {
        Command::Push => println!("You attempted to push!"),
        Command::Revert { .. } => println!("You attempted to revert!"),
    }
}
