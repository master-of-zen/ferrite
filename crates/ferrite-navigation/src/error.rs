use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NavigationError {
    #[error("Failed to access directory: {0}")]
    DirectoryAccess(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
}

pub type Result<T> = std::result::Result<T, NavigationError>;
