use anyhow::{Context, Result};
use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub borderless: bool,
    pub hide_menu: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            borderless: false,
            hide_menu: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeriteConfig {
    pub cache_size: usize,
    pub default_zoom: f32,
    pub show_performance: bool,
    pub recent_files: Vec<PathBuf>,
    pub max_recent_files: usize,
    pub zoom: ZoomConfig,
    pub window: WindowConfig,
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
    pub require_ctrl_for_zoom: bool,
    pub zoom_display_corner: Corner,
    pub show_zoom_level: bool,
}

impl Default for FeriteConfig {
    fn default() -> Self {
        Self {
            cache_size: 5,
            default_zoom: 1.0,
            show_performance: false,
            recent_files: Vec::new(),
            max_recent_files: 10,
            zoom: ZoomConfig::default(),
            window: WindowConfig::default(),
        }
    }
}

impl Default for Corner {
    fn default() -> Self {
        Corner::TopLeft
    }
}

impl Default for ZoomConfig {
    fn default() -> Self {
        Self {
            require_ctrl_for_zoom: false,
            zoom_display_corner: Corner::default(),
            show_zoom_level: true,
        }
    }
}

impl FeriteConfig {
    #[instrument]
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "No configuration file found at {:?}. Run with --generate-config to create one.",
                config_path
            ));
        }

        let config = Config::builder()
            .add_source(config::File::from_str(
                toml::to_string(&Self::default())?.as_str(),
                config::FileFormat::Toml,
            ))
            .add_source(File::from(config_path))
            .build()?;

        config
            .try_deserialize()
            .context("Failed to deserialize configuration")
    }

    #[instrument]
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let toml = toml::to_string_pretty(self).context("Failed to serialize configuration")?;
        std::fs::write(&config_path, toml).context("Failed to write configuration file")?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "ferrite", "ferrite")
            .context("Failed to determine project directories")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FeriteConfig::default();
        assert_eq!(config.cache_size, 5);
        assert_eq!(config.default_zoom, 1.0);
        assert!(!config.show_performance);
        assert!(config.recent_files.is_empty());
        assert_eq!(config.max_recent_files, 10);
    }

    #[test]
    fn test_config_serialization() {
        let config = FeriteConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: FeriteConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.cache_size, config.cache_size);
    }
}
