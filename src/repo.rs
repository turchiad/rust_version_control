// internal crates
use crate::ProjectError;
use crate::ProjectError::*;

/// struct for maintaining info about the repository
pub struct Repo {
    location: String,
    latest_version: Option<String>,
}

impl Repo {
    /// constructor for making a new repo
    pub fn new() -> Result<Repo, ProjectError> {
        match Repo::find_location()? {
            Some(location) => Repo::new_from_location(&location),
            None => Err(RepoNotFoundError),
        }
    }

    /// constructor for making a new repo given a provided location
    /// this method will check that the location is valid first before returning
    pub fn try_new_from_location(location: &str) -> Result<Repo, ProjectError> {
        match Repo::validate_location(location)? {
            true => Repo::new_from_location(location),
            false => Err(RepoInvalidLocationError(String::from(location))),
        }
    }

    /// constructor for making a new repo given a provided location
    /// this method will NOT check that the location is valid first before returning
    pub fn new_from_location(location: &str) -> Result<Repo, ProjectError> {
        Ok(Repo {
            location: String::from(location),
            latest_version: None,
        })
    }

    pub fn find_location() -> Result<Option<String>, ProjectError> {
        Err(UnimplementedError)
    }

    pub fn validate_location(location: &str) -> Result<bool, ProjectError> {
        Err(UnimplementedError)
    }

    pub fn next_version() -> Result<String, ProjectError> {
        Err(UnimplementedError)
    }
}
