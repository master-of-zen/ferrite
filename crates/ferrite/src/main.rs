use std::sync::Arc;

use eframe::{egui::ViewportBuilder, Error};
use ferrite_cache::{CacheConfig, CacheManager};
use ferrite_cli::Args;
use ferrite_core::FeriteApp;
use ferrite_logging::{init, LogConfig};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // just for now unwrap
    // TODO: HANDLE THIS CORRECTLY
    init(LogConfig {
        level:        args.get_log_level().unwrap(),
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

    let cache_config = CacheConfig::default();
    let cache_manager = Arc::new(CacheManager::new(cache_config));

    if let Some(ref image_path) = args.image_path {
        if let Err(e) = cache_manager.get_image(image_path.clone()) {
            eprintln!("Warning: Failed to pre-cache image: {}", e);
        }
    }

    let mut native_options = eframe::NativeOptions::default();

    // Use the width and height directly from config
    let width = config.window.width as f32;
    let height = config.window.height as f32;

    native_options.viewport = ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_decorations(!config.window.borderless);

    eframe::run_native(
        "Ferrite",
        native_options,
        Box::new(move |cc| {
            let app = Box::new(FeriteApp::new(
                cc,
                args.image_path,
                config,
                cache_manager,
            ));
            Ok(app)
        }),
    )
}
