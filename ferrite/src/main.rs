use eframe::Error;
use egui::ViewportBuilder;
use ferrite_cli::Args;
use ferrite_core::FeriteApp;
use ferrite_logging::{init, LogConfig};
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    init(LogConfig {
        level:        args.get_log_level(),
        enable_tracy: true,
        log_spans:    true,
    });

    let config = args.handle_config().unwrap_or_else(|e| {
        eprintln!(
            "Configuration error: {}. Run with --generate-config to create \
             one.",
            e
        );
        std::process::exit(1);
    });

    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Dark;

    let width: f32 = 1920.;
    let height: f32 = 1080.;

    native_options.viewport = ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_decorations(!config.window.borderless);

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| {
            let app =
                Box::new(FeriteApp::new(cc, args.image_path, config, runtime));
            app
        }),
    )
}
