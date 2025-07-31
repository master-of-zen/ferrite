use super::{
    defaults::window::*,
    error::{ConfigError, Result},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "default_width")]
    pub width:      u32,
    #[serde(default = "default_height")]
    pub height:     u32,
    pub borderless: bool,
}

fn default_width() -> u32 {
    DEFAULT_WIDTH
}

fn default_height() -> u32 {
    DEFAULT_HEIGHT
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width:      DEFAULT_WIDTH,
            height:     DEFAULT_HEIGHT,
            borderless: BORDERLESS,
        }
    }
}

impl WindowConfig {
    pub fn validate(&self) -> Result<()> {
        if self.width < MIN_WIDTH {
            return Err(ConfigError::ValidationError(format!(
                "Window width must be at least {}",
                MIN_WIDTH
            )));
        }
        if self.height < MIN_HEIGHT {
            return Err(ConfigError::ValidationError(format!(
                "Window height must be at least {}",
                MIN_HEIGHT
            )));
        }
        Ok(())
    }

    pub fn with_dimensions(width: u32, height: u32) -> Result<Self> {
        let config = Self {
            width,
            height,
            ..Self::default()
        };

        config.validate()?;
        Ok(config)
    }
}
