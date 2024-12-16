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
    pub hide_menu:  bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            dimensions: None, borderless: BORDERLESS, hide_menu: HIDE_MENU
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_dimensions_validation() {
        assert!(WindowDimensions::new(MIN_WIDTH, MIN_HEIGHT).is_ok());
        assert!(WindowDimensions::new(MIN_WIDTH - 1, MIN_HEIGHT).is_err());
        assert!(WindowDimensions::new(MIN_WIDTH, MIN_HEIGHT - 1).is_err());
    }

    #[test]
    fn test_default_config() {
        let config = WindowConfig::default();
        assert!(config.validate().is_ok());
        assert!(config.dimensions.is_none());
        assert_eq!(config.borderless, BORDERLESS);
        assert_eq!(config.hide_menu, HIDE_MENU);
    }
}
