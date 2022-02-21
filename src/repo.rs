// standard crates
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// internal crates
use crate::ProjectError;
use crate::ProjectError::*;

const REPO_ROOT: &str = ".rvc";
const CONFIG_NAME: &str = "config";
const ABSOLUTE_ROOT: &str = "/"

// Functions for non-primitive constants

/// constructing non-primitive constant REPO_ROOT
fn const_repo_root() -> PathBuf {
    PathBuf::from(REPO_ROOT)
}

/// constructing non_primitive constant CONFIG_NAME
fn const_config_name() -> PathBuf {
    PathBuf::from(CONFIG_NAME)
}

/// constructing non_primitive constant ABSOLUTE_ROOT
fn const_absolute_root() -> PathBuf {
    PathBuf::from(ABSOLUTE_ROOT)
}

/// struct for maintaining info about the repository
pub struct Repo {
    /// stores the path of the root of the repository, which should be `working_directory_root_path/REPO_ROOT`
    path_root_repo_dir: PathBuf,
    /// stores the path of the root of the working directory, which should be `../repo_root_path`
    path_root_working_dir: PathBuf,
    /// stores the path of the config file, which should be `repo_root_path/config`
    path_config: PathBuf,
    /// stores the latest version number (if any) of the repo
    latest_version: Option<String>,
}

impl Repo {
    /// flexible constructor for the Repo struct
    /// this method will attempt to find an existing valid repository
    /// and will return a Repo struct configured to that
    /// repository if found, otherwise
    /// this method will attempt to create a new repository
    /// in the current directory and will return a Repo struct configured to that
    /// repoository if successful
    pub fn try_new() -> Result<Repo, ProjectError> {
        Err(UnimplementedError)
    }

    /// default constructor for Repo struct
    /// this method will attempt to create a new repository
    /// in the current directory and will return a Repo struct configured to that
    /// repository if successful
    pub fn new() -> Result<Repo, ProjectError> {
        let path_root_working_dir =
            env::current_dir().map_err(|_| UnexpectedCurrentDirectoryNotFoundError)?;

        Repo::new_from_path(&path_root_working_dir)
    }

    /// default constructor for Repo struct
    /// this method will attempt to create a new repository
    /// in the given directory and will return a Repo struct configured to that
    pub fn new_from_path(path_root_working_dir: &Path) -> Result<Repo, ProjectError> {
        // First, initialize all expected instance variables
        let path_root_working_dir = PathBuf::from(path_root_working_dir);
        let path_root_repo_dir = path_root_working_dir.join(const_repo_root());
        let path_config = path_root_repo_dir.join(const_config_name());
        // Next, create `path_root_repo_dir` and `path_config`
        Repo::setup(&path_root_repo_dir, &path_config)?;
        // Return constructed struct matching newly setup Repo
        Ok(Repo {
            path_root_repo_dir: path_root_repo_dir,
            path_root_working_dir: path_root_working_dir,
            path_config: path_config,
            latest_version: None,
        })
    }

    /// alternative constructor for Repo struct
    /// this method will attempt to find an existing valid repository
    /// and will return a Repo struct configured to that
    /// repository if found
    pub fn new_from_existing() -> Result<Repo, ProjectError> {
        Err(UnimplementedError)
    }

    /// alternative constructor for Repo struct
    /// this method will verify if the given path `dir` points to a valid
    /// existing repository and will return a Repo struct configured to that
    /// repository if found
    pub fn new_from_existing_path() -> Result<Repo, ProjectError> {
        Err(UnimplementedError)
    }

    /// method to create a new repository directory and config file
    /// without returning any corresponding Repo struct
    fn setup(path_root_repo_dir: &Path, path_config: &Path) -> Result<(), ProjectError> {
        // First setup `path_root_repo_dir`
        Repo::setup_repo_dir(&path_root_repo_dir)?;
        // Next setup `path_config`
        Repo::setup_config(&path_config)?;
        Ok(())
    }

    /// method to create a new repository directory
    /// without returning any corresponding Repo struct
    fn setup_repo_dir(path_root_repo_dir: &Path) -> Result<(), ProjectError> {
        match path_root_repo_dir.is_dir() {
            // this method should not be called on an existing repo, so this is an error case
            true => Err(RepoRootAlreadyExistsError(PathBuf::from(
                path_root_repo_dir,
            ))),
            // attempt to create the repo directory
            false => match fs::create_dir(&path_root_repo_dir) {
                Ok(_) => Ok(()),
                Err(_) => Err(CreateDirectoryError(PathBuf::from(path_root_repo_dir))),
            },
        }
    }

    /// method to create a new config file
    /// without returning any corresponding Repo struct
    fn setup_config(path_config: &Path) -> Result<(), ProjectError> {
        // If we have reached this line, then path_config shouldn't exist, but it doesn't hurt to check
        match path_config.is_file() {
            // this method should not be called on an existing repo, so this is an error case
            true => Err(RepoConfigAlreadyExistsError(PathBuf::from(path_config))),
            // attempt to create the config file
            false => match fs::File::create(&path_config) {
                Ok(_) => Ok(()),
                Err(_) => Err(CreateFileError(PathBuf::from(path_config))),
            },
        }
    }

    /// this method will ascend through directories all the way to root to look for
    /// a repository that matches the repo nomenclature, will validate if this
    /// repo is a valid construction, and will return a Repo struct configured
    /// to that repository if found
    pub fn find_repo(absolute_root: Option<&Path>) -> Result<PathBuf, ProjectError> {
        // First, get any defaults
        let absolute_root = match absolute_root {
            Some(path_buf) => path_buf,
            None => const_absolute_root(),
        }
        // Next, get our current working directory in absolute terms
        let current_working_directory =
            env::current_dir().map_err(|_| UnexpectedCurrentDirectoryNotFoundError)?;
        let current_working_directory_absolute = current_working_directory
            .canonicalize()
            .map_err(|_| UnableToCanonicalizeError(current_working_directory));
        // We will use this to iterate upwards to search for the repo root
        let mut search_directory = PathBuf::from(current_working_directory_absolute);
        // Iterate through directories
        // while search_directory != absolute_root {
        //     match search_directory.join(const_repo_root).is_dir() {
        //         Ok(_) => 
        //     }
        // }

        Err(UnimplementedError)
    }

    /// this method will verify if the given path matches all of the necessary
    /// information to be considered a valid repository
    pub fn validate_path_is_repo() -> Result<bool, ProjectError> {
        Err(UnimplementedError)
    }
}
