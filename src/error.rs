use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("encountered an unimplemented method")]
    UnimplementedError,
}
