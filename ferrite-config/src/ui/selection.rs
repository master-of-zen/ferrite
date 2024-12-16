use crate::{
    defaults::selection::*,
    error::{ConfigError, Result},
    types::{ColorRGBA, MouseButton},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionConfig {
    pub enabled:             bool,
    pub show_box:            bool,
    pub trigger_button:      MouseButton,
    pub zoom_to_longer_side: bool,
    pub box_color:           ColorRGBA,
    pub box_thickness:       f64,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            enabled:             ENABLED,
            show_box:            SHOW_BOX,
            trigger_button:      TRIGGER_BUTTON,
            zoom_to_longer_side: ZOOM_TO_LONGER_SIDE,
            box_color:           ColorRGBA::new(
                BOX_COLOR.0,
                BOX_COLOR.1,
                BOX_COLOR.2,
                BOX_COLOR.3,
            ),
            box_thickness:       BOX_THICKNESS,
        }
    }
}
impl SelectionConfig {
    pub fn validate(&self) -> Result<()> {
        if self.box_thickness <= 0.0 {
            return Err(ConfigError::ValidationError(
                "Box thickness must be positive".into(),
            ));
        }
        Ok(())
    }
}
