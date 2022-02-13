// external crates
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("encountered an unimplemented method")]
    UnimplementedError,
    #[error("attempted to make repo with invalid location: {0}")]
    RepoInvalidLocationError(String),
    #[error("no valid location was found when trying to make repo")]
    RepoNotFoundError,
}
