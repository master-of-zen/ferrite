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

pub use ferrite_cache as cache;
pub use ferrite_cli as cli;
pub use ferrite_config as config;
pub use ferrite_core as core;
pub use ferrite_image as image;
pub use ferrite_logging as logging;
pub use ferrite_navigation as navigation;
pub use ferrite_ui as ui;

// Re-export main application components for convenience
pub use ferrite_cli::{Args, CliError};
pub use ferrite_config::FerriteConfig;
pub use ferrite_core::FeriteApp;
pub use ferrite_logging::{LogConfig, LogLevel};

// Re-export key types that users might need
pub use ferrite_cache::{CacheConfig, CacheManager};
pub use ferrite_image::SupportedFormats;
pub use ferrite_ui::{FitMode, ZoomHandler};
