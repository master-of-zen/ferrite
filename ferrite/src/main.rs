use eframe::Error;
use egui::ViewportBuilder;
use ferrite_cli::Args;
use ferrite_core::FeriteApp;
use ferrite_logging::{init, LogConfig};

fn main() -> Result<(), Error> {
    // Now Args::parse() will work correctly
    let args = Args::parse();

    // Initialize logging
    init(LogConfig {
        level:        args.get_log_level(),
        enable_tracy: true,
        log_spans:    true,
    });

    // Handle configuration
    let mut config = args.handle_config().unwrap_or_else(|e| {
        eprintln!(
            "Configuration error: {}. Run with --generate-config to create \
             one.",
            e
        );
        std::process::exit(1);
    });

    // Configure native window options based on config
    let mut native_options = eframe::NativeOptions::default();

    // Set window options
    native_options.default_theme = eframe::Theme::Dark;

    // Set initial window size if configured

    let width: f32 = 1920.;
    let height: f32 = 1080.;

    native_options.viewport = ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_decorations(!config.window.borderless);

    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| {
            let app = FeriteApp::new(cc, args.image_path, config);
            Box::new(app)
        }),
    )
}
