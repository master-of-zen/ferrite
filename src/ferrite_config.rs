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
}

impl Default for FeriteConfig {
    fn default() -> Self {
        Self {
            cache_size: 5,
            default_zoom: 1.0,
            show_performance: false,
            recent_files: Vec::new(),
            max_recent_files: 10,
        }
    }
}

impl FeriteConfig {
    /// Loads the configuration from disk, creating default config if none exists
    #[instrument]
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        // If config file doesn't exist, create it with default values
        if !config_path.exists() {
            info!("No config file found, creating default configuration");
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
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

    /// Adds a file to the recent files list, maintaining the maximum size
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove the path if it already exists to avoid duplicates
        self.recent_files.retain(|p| p != &path);

        // Add the new path at the beginning
        self.recent_files.insert(0, path);

        // Truncate to maximum size
        if self.recent_files.len() > self.max_recent_files {
            self.recent_files.truncate(self.max_recent_files);
        }
    }

    /// Gets the path to the configuration file
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "ferrite", "ferrite")
            .context("Failed to determine project directories")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}
