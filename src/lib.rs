// module declarations
pub mod args;
pub mod error;

// internal crates
use crate::args::Command;
use crate::error::ProjectError;

pub fn run(command: Command) -> Result<(), ProjectError> {
    // Separate by command
    match command {
        Command::Init => println!("You attempted to initialize!"),
        Command::Push => println!("You attempted to push!"),
        Command::Revert { .. } => println!("You attempted to revert!"),
    }

    Ok(())
}
