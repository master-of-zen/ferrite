use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use tracing::{debug, info, warn};

use super::{
    error::{ConfigError, Result},
    input::ControlsConfig,
    window::WindowConfig,
    zoom::ZoomConfig,
    CacheConfig,
    HelpMenuConfig,
    IndicatorConfig,
    CONFIG_VERSION,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FerriteConfig {
    version:       String,
    pub window:    WindowConfig,
    pub zoom:      ZoomConfig,
    pub controls:  ControlsConfig,
    pub indicator: IndicatorConfig,
    pub help_menu: HelpMenuConfig,
    pub cache:     CacheConfig,
    pub log_file:  Option<PathBuf>,
}

impl Default for FerriteConfig {
    fn default() -> Self {
        info!("Creating default configuration");

        // Determine platform-specific default log file path
        let default_log_file_path =
            ProjectDirs::from("com", "ferrite", "ferrite").map(|proj_dirs| {
                let log_dir = proj_dirs.data_local_dir().join("logs"); // Or cache_dir(), or state_dir()
                                                                       // We'll store the absolute path in the config by default.
                                                                       // The user can change it to a relative path if they prefer.
                log_dir.join("ferrite.log")
            });

        // Fallback if ProjectDirs fails (should be rare)
        let log_file = default_log_file_path.or_else(|| {
            warn!(
                "Could not determine platform-specific log directory. \
                 Defaulting log file to 'ferrite.log' in CWD or next to \
                 config."
            );
            Some(PathBuf::from("ferrite.log")) // This will be CWD-relative if
                                               // config path resolution also
                                               // fails
        });

        Self {
            version: CONFIG_VERSION.to_string(),
            window: WindowConfig::default(),
            zoom: ZoomConfig::default(),
            controls: ControlsConfig::default(),
            indicator: IndicatorConfig::default(),
            help_menu: HelpMenuConfig::default(),
            cache: CacheConfig::default(),
            log_file, // Use the determined default log file path
        }
    }
}

impl FerriteConfig {
    /// Determines the configuration file path by checking:
    /// 1. FERRITE_CONF environment variable
    /// 2. Default XDG config path
    pub fn resolve_config_path() -> Result<PathBuf> {
        // First check environment variable
        if let Ok(env_path) = env::var("FERRITE_CONF") {
            let path = PathBuf::from(env_path);

            // Validate the path from environment variable
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    return Err(ConfigError::InvalidPath(format!(
                        "Directory {} from FERRITE_CONF does not exist",
                        parent.display()
                    )));
                }
            }

            return Ok(path);
        }

        // Fall back to default XDG config path
        Self::get_default_path()
    }

    /// Loads configuration using environment-aware path resolution
    pub fn load() -> Result<Self> {
        let config_path = Self::resolve_config_path()?;

        if !config_path.exists() {
            info!(
                "No configuration file found at {:?}, using defaults",
                config_path
            );
            return Ok(Self::default());
        }

        info!("Loading configuration from {:?}", config_path);
        Self::load_from_path(&config_path)
    }

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
