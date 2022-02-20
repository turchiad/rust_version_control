// standard crates
use std::env;
use std::path::Path;
use std::path::PathBuf;

// internal crates
use crate::ProjectError;
use crate::ProjectError::*;

const REPO_ROOT: &str = ".rvc";

/// struct for maintaining info about the repository
pub struct Repo {
    location: PathBuf,
    latest_version: Option<String>,
}

impl Repo {
    /// constructor for making a new repo struct
    /// this constructor will additionally setup a repo struct at the location if
    /// one is not already made
    pub fn setup(dir: Option<&Path>) -> Result<Repo, ProjectError> {
        // First, acquire dir as a PathBuf
        let dir = match dir {
            Some(path) => PathBuf::from(path), // If we are given a path, use that
            None => match env::current_dir() {
                // otherwise, use the current directory
                Ok(path_buf) => path_buf,
                Err(_) => return Err(UnexpectedCurrentDirectoryNotFoundError),
            },
        };
        // Next, check if the repo location suggested is valid and in use
        match Repo::validate_location(dir)? {  // Return error if path provided was invalid
            // if true, the repo is already in use, just initialize normally
            true => return Repo::new_from_location(dir),
            false =>  
        }
    }

    // /// constructor for making a new repo struct
    // pub fn new() -> Result<Repo, ProjectError> {
    //     match Repo::find_location()? {
    //         Some(location) => Repo::new_from_location(&location),
    //         None => Err(RepoNotFoundError),
    //     }
    // }

    // /// constructor for making a new repo struct given a provided location
    // /// this method will check that the location is valid first before returning
    // pub fn try_new_from_location(location: &str) -> Result<Repo, ProjectError> {
    //     match Repo::validate_location(location)? {
    //         true => Repo::new_from_location(location),
    //         false => Err(RepoInvalidLocationError(String::from(location))),
    //     }
    // }

    // /// constructor for making a new repo given a provided location
    // /// this method will NOT check that the location is valid first before returning
    // pub fn new_from_location(dir: Path) -> Result<Repo, ProjectError> {
    //     Ok(Repo {
    //         location: PathBuf::from(dir),
    //         latest_version: None,
    //     })
    // }

    pub fn find_location() -> Result<Option<String>, ProjectError> {
        Err(UnimplementedError)
    }

    /// This method checks if `dir` fits the repo nomenclature
    /// and also checks if `dir` is a valid directory
    ///
    /// # Returns
    ///     `Ok(true)`    -> if `dir` matches `REPO_ROOT` and exists
    ///     `Ok(false)`   -> if `dir` matches `REPO_ROOT` and does not exist
    ///     `Err(x)`      -> if `dir` does not match `REPO_ROOT`
    pub fn validate_location(dir: &Path) -> Result<bool, ProjectError> {
        // First, check if `dir` fits repo nomenclature
        match dir.ends_with(REPO_ROOT) {
            true => (),
            false => return Err(RepoInvalidFormatError(PathBuf::from(dir))),
        }
        // Next, check if `dir` is a valid directory
        // the result of this will be the return value
        Ok(dir.is_dir())
    }

    pub fn next_version() -> Result<String, ProjectError> {
        Err(UnimplementedError)
    }
}
