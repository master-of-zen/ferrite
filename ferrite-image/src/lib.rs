//! Image handling functionality for the Ferrite image viewer.
//!
//! This crate provides image loading, caching, and management capabilities.

mod error;
mod formats;
mod manager;

// Re-export primary types
pub use error::ImageError;
pub use formats::SupportedFormats;
pub use manager::ImageManager;
