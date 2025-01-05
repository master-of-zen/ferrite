use crate::types::Position;

// Window defaults
pub mod window {
    pub const MIN_WIDTH: u32 = 640;
    pub const MIN_HEIGHT: u32 = 480;
    pub const BORDERLESS: bool = true;
}

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
    pub const DEFAULT_FIT_MODE: &str = "FitLonger";
}

pub mod indicator {
    use super::*;

    pub const FONT_SIZE: f64 = 40.0;
    pub const FONT_FAMILY: &str = "system-ui";
    pub const BACKGROUND_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 128);
    pub const TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
    pub const PADDING_X: f64 = 5.0;
    pub const PADDING_Y: f64 = 5.0;
    pub const POSITION: Position = Position::TopRight;
    pub const SHOW_PERCENTAGE: bool = true;
}

pub mod help_menu {
    pub const FONT_SIZE: f64 = 24.0;
    pub const FONT_FAMILY: &str = "system-ui";
    pub const BACKGROUND_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 128);
    pub const TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
    pub const PADDING_X: f64 = 10.0;
    pub const PADDING_Y: f64 = 10.0;
}

pub mod controls {
    use egui::Key;

    pub const ZOOM_IN_KEYS: &[Key] = &[Key::Equals, Key::Plus, Key::W];
    pub const ZOOM_OUT_KEYS: &[Key] = &[Key::Minus, Key::S];
    pub const RESET_ZOOM_KEY: Key = Key::Num0;
    pub const TOGGLE_FIT_KEY: Key = Key::F;
    pub const QUIT_KEY: Key = Key::Q;
    pub const HELP_KEY: Key = Key::H;
}

pub mod navigation {

    pub const DEFAULT_SORT_ORDER: &str = "name";
    pub const DEFAULT_REVERSE_ORDER: bool = false;
}
