// module declarations
pub mod args;
pub mod error;

// internal crates
use crate::args::Command;
use crate::error::ProjectError;
use crate::error::ProjectError::*;

/// This method is the main runtime called when command-line arguments have successfully been passed
pub fn run(command: Command) -> Result<(), ProjectError> {
    // Separate by command
    match command {
        Command::Init => init()?,
        Command::Push => push()?,
        Command::Revert { version } => try_revert(&version)?,
    }

    Ok(())
}

/// This method is the runtime for an `init` command
fn init() -> Result<(), ProjectError> {
    println!("You attempted to initialize!");
    Err(UnimplementedError)
}

/// This method is the runtime for an `push` command
fn push() -> Result<(), ProjectError> {
    println!("You attempted to push!");
    Err(UnimplementedError)
}

/// This method first checks that the version number given is valid, before running `revert()`
fn try_revert(version: &str) -> Result<(), ProjectError> {
    validate_version(version)?;
    revert(version)
}

fn validate_version(version: &str) -> Result<(), ProjectError> {
    println!("You attempted to validate {version}!");
    Err(UnimplementedError)
}

/// This method is the runtime for an `revert` command
fn revert(version: &str) -> Result<(), ProjectError> {
    println!("You attempted to revert to {version}!");
    Err(UnimplementedError)
}
