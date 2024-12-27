use crate::{
    defaults::window::*,
    error::{ConfigError, Result},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowDimensions {
    pub width:  u32,
    pub height: u32,
}

impl WindowDimensions {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        if width < MIN_WIDTH {
            return Err(ConfigError::ValidationError(format!(
                "Window width must be at least {}",
                MIN_WIDTH
            )));
        }
        if height < MIN_HEIGHT {
            return Err(ConfigError::ValidationError(format!(
                "Window height must be at least {}",
                MIN_HEIGHT
            )));
        }
        Ok(Self {
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub dimensions: Option<WindowDimensions>,
    pub borderless: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            dimensions: None, borderless: BORDERLESS
        }
    }
}

impl WindowConfig {
    pub fn validate(&self) -> Result<()> {
        if let Some(ref dims) = self.dimensions {
            if dims.width < MIN_WIDTH {
                return Err(ConfigError::ValidationError(format!(
                    "Window width must be at least {}",
                    MIN_WIDTH
                )));
            }
            if dims.height < MIN_HEIGHT {
                return Err(ConfigError::ValidationError(format!(
                    "Window height must be at least {}",
                    MIN_HEIGHT
                )));
            }
        }
        Ok(())
    }

    pub fn with_dimensions(width: u32, height: u32) -> Result<Self> {
        Ok(Self {
            dimensions: Some(WindowDimensions::new(width, height)?),
            ..Self::default()
        })
    }
}
