mod app;
use app::FeriteApp;
use std::path::PathBuf;
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

    // Get the command line arguments, skipping the program name
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Convert the first argument to a PathBuf if it exists
    let initial_image = args.first().map(PathBuf::from);

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    // Start the main application with the initial image path
    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| Box::new(FeriteApp::new(cc, initial_image))),
    )
}
