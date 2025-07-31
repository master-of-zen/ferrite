//! # Ferrite Image Viewer
//!
//! A fast and efficient image viewer with focus on performance.
//!
//! ## Features
//!
//! - High-performance image loading and caching
//! - Smooth zooming and panning
//! - Directory-based navigation
//! - Configurable UI and controls
//! - Multiple image format support
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ferrite::*;
//!
//! // Create and run the application
//! let config = config::FerriteConfig::load().unwrap_or_default();
//! // ... application setup
//! ```

// Module declarations for the unified crate
pub mod cache;
pub mod cli;
pub mod config;
pub mod core;
pub mod image;
pub mod logging;
pub mod navigation;
pub mod ui;

// Re-export main application components for convenience
pub use cli::{Args, CliError};
pub use config::FerriteConfig;
pub use core::FeriteApp;
pub use logging::{LogConfig, LogLevel};

// Re-export key types that users might need
pub use cache::{CacheConfig, CacheManager};
pub use image::SupportedFormats;
pub use ui::{FitMode, ZoomHandler};
