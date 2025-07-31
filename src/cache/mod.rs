// Module: cache
pub mod manager;
pub mod types;

// Re-export main types from manager and types
pub use manager::*;
pub use types::*;

// Cache-specific error types
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Image loading error at {path}: {source}")]
    ImageLoad { path: std::path::PathBuf, source: ImageLoadError },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ImageLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Format error: {0}")]
    Format(String),
}

pub type CacheResult<T> = Result<T, CacheError>;
