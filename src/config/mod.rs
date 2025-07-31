// Internal modules
pub mod cache;
pub mod config;
pub mod defaults;
pub mod error;
pub mod help_menu;
pub mod indicator;
pub mod input;
pub mod navigation;
pub mod types;
pub mod window;
pub mod zoom;

// Re-exports
pub use cache::CacheConfig;
pub use config::FerriteConfig;
pub use error::{ConfigError, Result};
pub use help_menu::HelpMenuConfig;
pub use indicator::IndicatorConfig;
pub use input::ControlsConfig;
pub use window::WindowConfig;
pub use zoom::ZoomConfig;

pub use eframe::egui::{Color32, Key};
pub use types::{ColorRGBA, MouseButton, Position, Vector2D};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONFIG_VERSION: &str = "0.1";
