use egui::Key;
use serde::{Deserialize, Serialize};

use crate::{
    defaults::controls::*,
    error::{ConfigError, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub zoom_in_keys: Vec<Key>,
    pub zoom_out_keys: Vec<Key>,
    pub reset_zoom_key: Key,
    pub toggle_fit_key: Key,
    pub quit_key: Key,
    pub help_key: Key,
}

impl Default for ControlsConfig {
    fn default() -> Self {
        Self {
            zoom_in_keys: vec![Key::Equals, Key::Plus, Key::W],
            zoom_out_keys: vec![Key::Minus, Key::S],
            reset_zoom_key: Key::Num0,
            toggle_fit_key: Key::F,
            quit_key: Key::Q,
            help_key: Key::H,
        }
    }
}

impl ControlsConfig {
    pub fn validate(&self) -> Result<()> {
        if self.zoom_in_keys.is_empty() {
            return Err(ConfigError::ValidationError(
                "No zoom in keys defined".into(),
            ));
        }
        if self.zoom_out_keys.is_empty() {
            return Err(ConfigError::ValidationError(
                "No zoom out keys defined".into(),
            ));
        }

        // Check for key conflicts
        let mut all_keys = Vec::new();
        all_keys.extend(&self.zoom_in_keys);
        all_keys.extend(&self.zoom_out_keys);
        all_keys.push(self.reset_zoom_key);
        all_keys.push(self.toggle_fit_key);

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
