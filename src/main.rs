mod app;
mod cli;
mod ferrite_config;

use app::FeriteApp;
use cli::CliArgs;
use ferrite_config::FeriteConfig;
use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

#[instrument]
fn main() -> Result<(), eframe::Error> {
    // Initialize tracing with both console output and Tracy profiler
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_tracy::TracyLayer::new())
        .init();

    info!("Starting Ferrite image viewer");

    // Parse command line arguments
    let args = CliArgs::parse_args();

    // Load configuration and apply CLI overrides
    let mut config = FeriteConfig::load().expect("Failed to load configuration");
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
