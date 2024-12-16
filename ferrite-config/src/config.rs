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
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            debug!("No config file found at {:?}, using defaults", config_path);
            return Ok(Self::default());
        }

        info!("Loading configuration from {:?}", config_path);
        let content = fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&content)?;

        if config.version != CONFIG_VERSION {
            return Err(ConfigError::VersionError {
                found:     config.version,
                supported: CONFIG_VERSION.to_string(),
            });
        }

        config.validate()?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        self.validate()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        info!("Saved configuration to {:?}", config_path);
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        ProjectDirs::from("com", "ferrite", "ferrite")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
            .ok_or_else(|| ConfigError::DirectoryError(PathBuf::from(".")))
    }

    fn validate(&self) -> Result<()> {
        self.window.validate()?;
        self.zoom.validate()?;
        self.controls.validate()?;
        self.indicator.validate()?;
        self.selection.validate()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FerriteConfig::default();
        assert_eq!(config.version, CONFIG_VERSION);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_version_validation() {
        let mut config = FerriteConfig::default();
        config.version = "invalid".to_string();
        assert!(matches!(
            config.validate(),
            Err(ConfigError::VersionError { .. })
        ));
    }
}
