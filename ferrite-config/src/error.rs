//! Error handling for the Ferrite configuration system.
//!
//! This module defines the error types that can occur during configuration
//! operations. It uses the thiserror crate to provide detailed, context-aware
//! error messages that help users understand and fix configuration issues.

use std::path::PathBuf;
use thiserror::Error;

/// Represents all possible errors that can occur in the configuration system.
/// Each variant provides specific context about what went wrong.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Indicates an error occurred while reading or writing configuration
    /// files
    #[error("Failed to access configuration file: {0}")]
    IoError(#[from] std::io::Error),

    /// Indicates the configuration file couldn't be parsed as valid TOML
    #[error("Failed to parse configuration TOML: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Indicates an error occurred while serializing configuration to TOML
    #[error("Failed to serialize configuration to TOML: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    /// Indicates the configuration file wasn't found at the expected location
    #[error("Configuration file not found at: {0}")]
    FileNotFound(PathBuf),

    /// Indicates a validation error in the configuration values
    #[error("Invalid configuration value: {0}")]
    ValidationError(String),

    /// Indicates an unsupported configuration file version was detected
    #[error(
        "Unsupported configuration version {found} (supported: {supported})"
    )]
    VersionError { found: String, supported: String },

    /// Indicates a failure to create or access the configuration directory
    #[error("Failed to access configuration directory: {0}")]
    DirectoryError(PathBuf),

    /// Indicates an error in color value parsing or validation
    #[error("Invalid color value: {0}")]
    ColorError(String),

    /// Indicates an invalid keyboard or mouse button configuration
    #[error("Invalid input configuration: {0}")]
    InputError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Type alias for Result types using our ConfigError
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Provides testing utilities for error handling
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Test that error messages are human-readable
        let err = ConfigError::ValidationError(
            "Zoom level must be positive".to_string(),
        );
        assert_eq!(
            err.to_string(),
            "Invalid configuration value: Zoom level must be positive"
        );

        let err = ConfigError::VersionError {
            found:     "0.2".to_string(),
            supported: "0.1".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Unsupported configuration version 0.2 (supported: 0.1)"
        );
    }
}
