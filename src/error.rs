// standard crates
use std::path::PathBuf;

// external crates
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("encountered an unimplemented method")]
    UnimplementedError,
    #[error("attempted to make repo with invalid location: {0}")]
    RepoInvalidLocationError(PathBuf),
    #[error("attempted to make repo with invalid format: {0}")]
    RepoInvalidFormatError(PathBuf),
    #[error("no valid location was found when trying to make repo")]
    RepoNotFoundError,
    #[error("unexpected error: current working directory not found")]
    UnexpectedCurrentDirectoryNotFoundError,
}
