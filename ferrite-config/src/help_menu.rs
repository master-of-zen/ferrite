use crate::{
    defaults::help_menu::*,
    error::{ConfigError, Result},
    types::{ColorRGBA, Vector2D},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpMenuConfig {
    pub font_size:        f64,
    pub font_family:      String,
    pub background_color: ColorRGBA,
    pub text_color:       ColorRGBA,
    pub padding:          Vector2D,
}

impl Default for HelpMenuConfig {
    fn default() -> Self {
        Self {
            font_size:        FONT_SIZE,
            font_family:      FONT_FAMILY.to_string(),
            background_color: ColorRGBA::new(
                BACKGROUND_COLOR.0,
                BACKGROUND_COLOR.1,
                BACKGROUND_COLOR.2,
                BACKGROUND_COLOR.3,
            ),
            text_color:       ColorRGBA::new(
                TEXT_COLOR.0,
                TEXT_COLOR.1,
                TEXT_COLOR.2,
                TEXT_COLOR.3,
            ),
            padding:          Vector2D::new(PADDING_X, PADDING_Y)
                .expect("Default padding must be valid"),
        }
    }
}

impl HelpMenuConfig {
    pub fn validate(&self) -> Result<()> {
        if self.font_size <= 0.0 {
            return Err(ConfigError::ValidationError(
                "Font size must be positive".into(),
            ));
        }
        if self.font_family.trim().is_empty() {
            return Err(ConfigError::ValidationError(
                "Font family cannot be empty".into(),
            ));
        }
        Ok(())
    }
}
