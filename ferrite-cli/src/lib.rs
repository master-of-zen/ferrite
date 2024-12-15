use anyhow::Result;
use clap::Parser;
use ferrite_config::{Corner, FeriteConfig};
use ferrite_logging::LogLevel;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Ferrite - A fast and efficient image viewer")]
pub struct Args {
    /// Initial image file to open
    #[arg(value_name = "IMAGE")]
    pub image_path: Option<PathBuf>,

    /// Override the default cache size
    #[arg(short, long, value_name = "SIZE")]
    pub cache_size: Option<usize>,

    /// Set the initial zoom level (e.g., 1.0 for 100%)
    #[arg(short, long, value_name = "LEVEL")]
    pub zoom: Option<f32>,

    /// Show the performance monitoring window
    #[arg(short, long)]
    pub perf: bool,

    /// Override the maximum number of recent files
    #[arg(long, value_name = "COUNT")]
    pub max_recent: Option<usize>,

    /// Set the logging level
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: Option<String>,

    /// Set the corner for zoom level display
    #[arg(long, value_name = "CORNER")]
    pub zoom_corner: Option<String>,

    /// Toggle zoom level display
    #[arg(long)]
    pub hide_zoom: bool,

    /// Generate a default configuration file
    #[arg(long)]
    pub generate_config: bool,
}

impl Args {
    pub fn parse() -> Self {
        Self::parse()
    }

    pub fn get_log_level(&self) -> LogLevel {
        self.log_level
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(LogLevel::Info)
    }

    pub fn handle_config(&self) -> Result<FeriteConfig> {
        if self.generate_config {
            let config = FeriteConfig::default();
            config.save()?;
            Ok(config)
        } else {
            FeriteConfig::load()
        }
    }

    pub fn apply_to_config(&self, config: &mut FeriteConfig) {
        if let Some(cache_size) = self.cache_size {
            config.cache_size = cache_size;
        }
        if let Some(zoom) = self.zoom {
            config.default_zoom = zoom;
        }
        if self.perf {
            config.show_performance = true;
        }
        if let Some(max_recent) = self.max_recent {
            config.max_recent_files = max_recent;
        }
        if let Some(corner) = &self.zoom_corner {
            config.zoom.zoom_display_corner = match corner.to_lowercase().as_str() {
                "top-left" => Corner::TopLeft,
                "top-right" => Corner::TopRight,
                "bottom-left" => Corner::BottomLeft,
                "bottom-right" => Corner::BottomRight,
                _ => Corner::TopLeft,
            };
        }
        if self.hide_zoom {
            config.zoom.show_zoom_level = false;
        }
    }
}
