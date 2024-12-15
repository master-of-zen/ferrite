use anyhow::{Context, Result};
use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, instrument, warn};

/// Represents the application's configuration settings
#[derive(Debug, Serialize, Deserialize)]
pub struct FeriteConfig {
    /// Maximum number of images to keep in the LRU cache
    pub cache_size: usize,
    /// Default zoom level for newly opened images
    pub default_zoom: f32,
    /// Whether to show the performance window by default
    pub show_performance: bool,
    /// List of recently opened files
    pub recent_files: Vec<PathBuf>,
    /// Maximum number of recent files to remember
    pub max_recent_files: usize,
    /// Zoom-related configuration
    pub zoom: ZoomConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoomConfig {
    /// Whether Ctrl key is required for zoom
    pub require_ctrl_for_zoom: bool,
    /// Corner where zoom level is displayed
    pub zoom_display_corner: Corner,
    /// Whether to show zoom level
    pub show_zoom_level: bool,
}

impl FeriteConfig {
    /// Loads the configuration from disk
    #[instrument]
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        // If config file doesn't exist, return an error
        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "No configuration file found at {:?}. Run with --generate-config to create one.",
                config_path
            ));
        }

        info!("Loading configuration from {:?}", config_path);

        // Build configuration from multiple sources, with each subsequent
        // source overriding values from previous ones
        let config = Config::builder()
            // Start with default values
            .add_source(config::File::from_str(
                toml::to_string(&Self::default())?.as_str(),
                config::FileFormat::Toml,
            ))
            // Override with user's config file
            .add_source(File::from(config_path))
            // Could add more sources here (e.g., environment variables)
            .build()?;

        // Deserialize the config into our structure
        let config: FeriteConfig = config
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        Ok(config)
    }

    /// Saves the current configuration to disk
    #[instrument]
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // Serialize and save the config
        let toml = toml::to_string_pretty(self).context("Failed to serialize configuration")?;
        std::fs::write(&config_path, toml).context("Failed to write configuration file")?;

        info!("Configuration saved to {:?}", config_path);
        Ok(())
    }

    /// Gets the path to the configuration file
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "ferrite", "ferrite")
            .context("Failed to determine project directories")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}
