use eframe::Error;
use egui::ViewportBuilder;
use ferrite_cli::Args;
use ferrite_logging::{init, LogConfig};
use tracing::instrument;

mod app;

#[instrument]
fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Initialize logging
    init(LogConfig {
        level: args.get_log_level(),
        enable_tracy: true,
    });

    // Handle configuration
    let mut config = args.handle_config().unwrap_or_else(|e| {
        eprintln!(
            "Configuration error: {}. Run with --generate-config to create one.",
            e
        );
        std::process::exit(1);
    });
    args.apply_to_config(&mut config);

    // Configure native window options based on config
    let mut native_options = eframe::NativeOptions::default();

    // Set window options
    native_options.default_theme = eframe::Theme::Dark;

    // Set initial window size if configured
    if let (Some(width), Some(height)) = (config.window.width, config.window.height) {
        native_options.viewport = ViewportBuilder::default()
            .with_inner_size([width as f32, height as f32])
            .with_decorations(!config.window.borderless);
    } else {
        // If no size specified, just set decorations
        native_options.viewport =
            ViewportBuilder::default().with_decorations(!config.window.borderless);
    }

    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| {
            let app = app::FeriteApp::new(cc, args.image_path, config);
            Box::new(app)
        }),
    )
}
