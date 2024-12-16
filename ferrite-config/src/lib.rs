//! Ferrite Configuration Management
//!
//! This crate provides a type-safe, validated configuration system for the
//! Ferrite image viewer. It handles configuration loading, saving, and
//! validation through a TOML-based file system.
//!
//! # Features
//! - Type-safe configuration with validation
//! - TOML-based storage
//! - Standard configuration paths
//! - Version tracking for compatibility
//! - Comprehensive error handling
//!
//! # Example
//! ```rust,no_run
//! use ferrite_config::FerriteConfig;
//!
//! // Load existing configuration or create default
//! let config = FerriteConfig::load().unwrap_or_default();
//!
//! // Access settings
//! println!("Window borderless: {}", config.window.borderless);
//! println!("Zoom level: {}", config.zoom.max_zoom);
//! ```

// Re-export primary types for user convenience
pub use config::FerriteConfig;
pub use error::{ConfigError, Result};

// Re-export configuration component types
pub use input::ControlsConfig;
pub use ui::{IndicatorConfig, SelectionConfig};
pub use window::WindowConfig;
pub use zoom::ZoomConfig;

// Re-export common types used in configuration
pub use types::{Color32, ColorRGBA, Corner, Key, MouseButton, Vector2D};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONFIG_VERSION: &str = "0.1";

// Internal modules
mod config;
mod defaults;
mod error;
mod input;
mod types;
mod ui;
mod window;
mod zoom;
