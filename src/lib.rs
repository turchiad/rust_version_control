// module declarations
pub mod args;
pub mod error;
pub mod repo;

// internal crates
use crate::args::Command;
use crate::error::ProjectError;
use crate::error::ProjectError::*;
use crate::repo::Repo;

/// This method is the main runtime called when command-line arguments have successfully been passed
pub fn run(command: Command) -> Result<(), ProjectError> {
    // Separate by command
    match command {
        Command::Init => init()?,
        Command::Push => try_push()?,
        Command::Revert { version } => try_revert(&version)?,
    }

    Ok(())
}

/// this method is the runtime for an `init` command
fn init() -> Result<(), ProjectError> {
    println!("You attempted to initialize!");
    Err(UnimplementedError)
}

fn read_repo() -> Result<Repo, ProjectError> {
    Repo::new()
}

/// this method
/// 1. checks the repository is initialized and creates a Repo
/// 2. attempts to push a new version of the project to the Repo
fn try_push() -> Result<(), ProjectError> {
    let repo = read_repo()?;
    push(&repo)
}

/// This method is the runtime for an `push` command
fn push(repo: &Repo) -> Result<(), ProjectError> {
    println!("You attempted to push!");
    Err(UnimplementedError)
}

/// this method
/// 1. checks the repository is initialized and creates a Repo
/// 2. validates the version number provided exists
/// 3. attempts to revert the project to this version
fn try_revert(version: &str) -> Result<(), ProjectError> {
    let repo = read_repo()?;
    validate_version(&repo, version)?;
    revert(&repo, version)
}

/// this method validates that the version number provided exists
fn validate_version(repo: &Repo, version: &str) -> Result<(), ProjectError> {
    println!("You attempted to validate {version}!");
    Err(UnimplementedError)
}

/// this method is the runtime for an `revert` command
fn revert(repo: &Repo, version: &str) -> Result<(), ProjectError> {
    println!("You attempted to revert to {version}!");
    Err(UnimplementedError)
}
