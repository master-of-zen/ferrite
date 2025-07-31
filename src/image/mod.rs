//! Image handling functionality for the Ferrite image viewer.
//!
//! This crate provides image loading, caching, and management capabilities.

pub mod error;
pub mod formats;
pub mod loaders;
pub mod manager;
pub mod operations;

// Re-export primary types
pub use error::ImageError;
pub use formats::SupportedFormats;
pub use loaders::ImageLoader;
pub use manager::ImageManager;
pub use operations::{FileOperationError, FileOperations};
