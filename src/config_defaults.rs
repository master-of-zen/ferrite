// Contains default values for all configuration settings in the application
use crate::ferrite_config::{Corner, FeriteConfig, ZoomConfig};
use std::path::PathBuf;

impl Default for FeriteConfig {
    fn default() -> Self {
        Self {
            // Default to keeping 5 images in the LRU cache - a balance between
            // memory usage and performance when navigating between images
            cache_size: 5,

            // Start with 1.0 (100%) zoom by default for a natural initial view
            default_zoom: 1.0,

            // Performance window hidden by default to keep the interface clean
            show_performance: false,

            // Start with an empty list of recent files
            recent_files: Vec::new(),

            // Remember the last 10 files opened - enough for quick access without
            // cluttering the menu
            max_recent_files: 10,

            // Initialize zoom-related settings with defaults
            zoom: ZoomConfig::default(),
        }
    }
}

impl Default for Corner {
    fn default() -> Self {
        // Top-left is a common location for UI elements and matches most
        // application conventions
        Corner::TopLeft
    }
}

impl Default for ZoomConfig {
    fn default() -> Self {
        Self {
            // Don't require Ctrl for zoom by default to make zooming more
            // accessible and match common image viewer behavior
            require_ctrl_for_zoom: false,

            // Use the default corner (top-left) for zoom display
            zoom_display_corner: Corner::default(),

            // Show zoom level by default to help users track their view scale
            show_zoom_level: true,
        }
    }
}
