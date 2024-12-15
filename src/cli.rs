use clap::Parser;
use std::path::PathBuf;
use tracing::Level;

/// Ferrite - A fast and efficient image viewer
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Initial image file to open
    #[arg(value_name = "IMAGE")]
    pub image_path: Option<PathBuf>,

    /// Override the default cache size (number of images to keep in memory)
    #[arg(short, long, value_name = "SIZE")]
    pub cache_size: Option<usize>,

    /// Set the initial zoom level (e.g., 1.0 for 100%, 2.0 for 200%)
    #[arg(short, long, value_name = "LEVEL")]
    pub zoom: Option<f32>,

    /// Show the performance monitoring window
    #[arg(short, long)]
    pub perf: bool,

    /// Override the maximum number of recent files to remember
    #[arg(long, value_name = "COUNT")]
    pub max_recent: Option<usize>,

    /// Set the logging level (trace, debug, info, warn, error)    
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: Option<String>,
}

impl CliArgs {
    /// Parse command line arguments and return the parsed structure
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Apply CLI arguments to the provided configuration
    pub fn apply_to_config(&self, config: &mut crate::ferrite_config::FeriteConfig) {
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
    }

    /// Convert string log level to tracing::Level
    pub fn parse_log_level(&self) -> Level {
        // First check CLI argument
        if let Some(level_str) = &self.log_level {
            match level_str.to_lowercase().as_str() {
                "trace" => Level::TRACE,
                "debug" => Level::DEBUG,
                "info" => Level::INFO,
                "warn" => Level::WARN,
                "error" => Level::ERROR,
                _ => Level::INFO, // Default to INFO for invalid values
            }
        } else {
            // If no CLI argument, check environment variable
            std::env::var("RUST_LOG")
                .ok()
                .and_then(|env_level| match env_level.to_lowercase().as_str() {
                    "trace" => Some(Level::TRACE),
                    "debug" => Some(Level::DEBUG),
                    "info" => Some(Level::INFO),
                    "warn" => Some(Level::WARN),
                    "error" => Some(Level::ERROR),
                    _ => None,
                })
                .unwrap_or(Level::INFO) // Default to INFO if env var is not set or invalid
        }
    }
}
