mod app;
mod cli;
mod config_defaults;
mod ferrite_config; // Add the new module

use app::FeriteApp;
use cli::CliArgs;
use ferrite_config::FeriteConfig;
use tracing::level_filters::LevelFilter;
use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

#[instrument]
fn main() -> Result<(), eframe::Error> {
    // Parse command line arguments first
    let args = CliArgs::parse_args();

    // Initialize tracing with both console output and Tracy profiler
    let log_level = args.parse_log_level();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::from_level(log_level)))
        .with(tracing_tracy::TracyLayer::new().with_filter(LevelFilter::from_level(log_level)))
        .init();

    info!(
        "Starting Ferrite image viewer with log level: {}",
        log_level
    );

    // Handle configuration initialization and CLI overrides
    let mut config = args.handle_config().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}. Run with --generate-config to create a default configuration.", e);
        std::process::exit(1);
    });
    args.apply_to_config(&mut config);

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    // Start the main application with the CLI arguments
    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| Box::new(FeriteApp::new(cc, args.image_path))),
    )
}
