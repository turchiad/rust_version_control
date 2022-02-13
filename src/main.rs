// standard crates
use std::process;

// internal crates
use rvc::args::*;

// external crates
use clap::Parser;

fn main() {
    // Obtain arguments
    let args = Args::parse();

    // Run runtime and exit out if error is encountered
    if let Err(e) = rvc::run(args.command) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
