use serde::Serialize;

use crate::{
    defaults::controls::*,
    error::{ConfigError, Result},
    types::SerializableKey,
};

#[derive(Debug, Clone, Serialize)]
pub struct ControlsConfig {
    pub zoom_in_keys:   Vec<SerializableKey>,
    pub zoom_out_keys:  Vec<SerializableKey>,
    pub reset_zoom_key: SerializableKey,
    pub toggle_fit_key: SerializableKey,
    pub quit_key:       SerializableKey,
}
impl Default for ControlsConfig {
    fn default() -> Self {
        Self {
            zoom_in_keys:   ZOOM_IN_KEYS.iter().map(|&s| s.into()).collect(),
            zoom_out_keys:  ZOOM_OUT_KEYS.iter().map(|&s| s.into()).collect(),
            reset_zoom_key: RESET_ZOOM_KEY.into(),
            toggle_fit_key: TOGGLE_FIT_KEY.into(),
            quit_key:       QUIT_KEY.into(),
        }
    }
}

impl ControlsConfig {
    pub fn validate(&self) -> Result<()> {
        if self.zoom_in_keys.as_slice().is_empty() {
            return Err(ConfigError::ValidationError(
                "No zoom in keys defined".into(),
            ));
        }
        if self.zoom_out_keys.as_slice().is_empty() {
            return Err(ConfigError::ValidationError(
                "No zoom out keys defined".into(),
            ));
        }

        // Check for key conflicts
        let mut all_keys = Vec::new();
        all_keys.extend(self.zoom_in_keys.as_slice());
        all_keys.extend(self.zoom_out_keys.as_slice());
        all_keys.push(&self.reset_zoom_key);
        all_keys.push(&self.toggle_fit_key);

        let mut seen = Vec::new();
        for key in all_keys {
            if seen.contains(&key) {
                return Err(ConfigError::ValidationError(format!(
                    "Duplicate key binding found: {:?}",
                    key
                )));
            }
            seen.push(key);
        }

        Ok(())
    }
}
