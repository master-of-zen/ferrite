use std::io;
use thiserror::Error;

/// Represents all possible errors that can occur during image operations
#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Failed to access image file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to decode or process image: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Invalid image path: {0}")]
    InvalidPath(String),

    #[error("Cache error: {0}")]
    CacheError(#[from] crate::cache::CacheError),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ImageError>;
