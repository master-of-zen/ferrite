use std::path::PathBuf;
use thiserror::Error;

mod manager;
mod types;

pub use manager::CacheManager;
pub use types::{CacheConfig, ImageData};

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Failed to load image from {path}: {source}")]
    ImageLoad { path: PathBuf, source: ImageLoadError },

    #[error("Cache capacity reached ({current} images, maximum {maximum})")]
    CapacityExceeded { current: usize, maximum: usize },

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Error, Debug)]
pub enum ImageLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid image format: {0}")]
    Format(String),
}

pub type CacheResult<T> = Result<T, CacheError>;
pub type LoadResult<T> = Result<T, ImageLoadError>;

#[cfg(feature = "ferrite-metrics")]
pub mod metrics {
    use ferrite_logging::metrics::PerformanceMetrics;
}

#[cfg(not(feature = "ferrite-metrics"))]
pub mod metrics {}

#[cfg(test)]
mod tests {
    
}
