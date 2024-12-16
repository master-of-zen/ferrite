use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::{debug, info, warn};

use crate::{
    error::{ConfigError, Result},
    input::ControlsConfig,
    ui::{IndicatorConfig, SelectionConfig},
    window::WindowConfig,
    zoom::ZoomConfig,
    CONFIG_VERSION,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FerriteConfig {
    version:       String,
    pub window:    WindowConfig,
    pub zoom:      ZoomConfig,
    pub controls:  ControlsConfig,
    pub indicator: IndicatorConfig,
    pub selection: SelectionConfig,
}

impl Default for FerriteConfig {
    fn default() -> Self {
        info!("Creating default configuration");
        Self {
            version:   CONFIG_VERSION.to_string(),
            window:    WindowConfig::default(),
            zoom:      ZoomConfig::default(),
            controls:  ControlsConfig::default(),
            indicator: IndicatorConfig::default(),
            selection: SelectionConfig::default(),
        }
    }
}
impl FerriteConfig {
    // Core configuration management methods
    pub fn load_from_path(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            debug!("No config file found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        info!("Loading configuration from {:?}", path);
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;

        if config.version != CONFIG_VERSION {
            return Err(ConfigError::VersionError {
                found:     config.version.clone(),
                supported: CONFIG_VERSION.to_string(),
            });
        }

        config.validate()?;
        Ok(config)
    }

    pub fn save_to_path(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        self.validate()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;

        info!("Saved configuration to {:?}", path);
        Ok(())
    }

    // Default paths handling
    pub fn get_default_path() -> Result<PathBuf> {
        ProjectDirs::from("com", "ferrite", "ferrite")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
            .ok_or_else(|| ConfigError::DirectoryError(PathBuf::from(".")))
    }

    // Configuration validation
    pub fn validate(&self) -> Result<()> {
        self.window.validate()?;
        self.zoom.validate()?;
        self.controls.validate()?;
        self.indicator.validate()?;
        self.selection.validate()?;
        Ok(())
    }

    // Utility method for creating new configurations
    pub fn with_modifications<F>(&self, modify_fn: F) -> Result<Self>
    where
        F: FnOnce(&mut Self),
    {
        let mut new_config = self.clone();
        modify_fn(&mut new_config);
        new_config.validate()?;
        Ok(new_config)
    }
}
