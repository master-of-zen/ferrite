use crate::types::{ColorRGBA, Corner, Key, MouseButton, Vector2D};

// Window defaults
pub mod window {
    pub const MIN_WIDTH: u32 = 200;
    pub const MIN_HEIGHT: u32 = 200;
    pub const BORDERLESS: bool = false;
    pub const HIDE_MENU: bool = false;
}

// Zoom defaults
pub mod zoom {
    pub const MIN_ZOOM: f64 = 0.1;
    pub const MAX_ZOOM: f64 = 10.0;
    pub const DEFAULT_ZOOM: f64 = 1.0;
    pub const ZOOM_STEP: f64 = 0.1;
    pub const DEFAULT_ZOOM_STEPS: &[f64] =
        &[0.1, 0.25, 0.5, 1.0, 2.0, 4.0, 8.0];
    pub const USE_PREDEFINED_STEPS: bool = false;
    pub const FOCAL_POINT_ENABLED: bool = true;
    pub const TRANSITION_ENABLED: bool = false;
    pub const TRANSITION_DURATION: f64 = 0.2;
    pub const FIT_TO_WINDOW: bool = true;
    pub const MAINTAIN_ASPECT_RATIO: bool = true;
}

// UI defaults
// defaults.rs
pub mod indicator {
    use super::*;

    pub const FONT_SIZE: f64 = 14.0;
    pub const FONT_FAMILY: &str = "system-ui";
    // Replace ColorRGBA constants with tuples
    pub const BACKGROUND_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 128);
    pub const TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
    pub const PADDING_X: f64 = 5.0;
    pub const PADDING_Y: f64 = 5.0;
    pub const CORNER: Corner = Corner::TopRight;
    pub const SHOW_PERCENTAGE: bool = true;
}

pub mod selection {
    use super::*;

    pub const ENABLED: bool = true;
    pub const SHOW_BOX: bool = true;
    pub const TRIGGER_BUTTON: MouseButton = MouseButton::Right;
    pub const ZOOM_TO_LONGER_SIDE: bool = true;
    pub const BOX_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 128);
    pub const BOX_THICKNESS: f64 = 1.0;
}

pub mod controls {
    use super::*;

    pub const ZOOM_IN_KEYS: &[&str] = &["Equal", "Plus", "W"];
    pub const ZOOM_OUT_KEYS: &[&str] = &["Minus", "S"];
    pub const RESET_ZOOM_KEY: &str = "Num0";
    pub const TOGGLE_FIT_KEY: &str = "F";
}
