use std::sync::Arc;

use eframe::{egui::ViewportBuilder, Error};
// Use ferrite_cache::CacheConfig for CacheManager constructor
use ferrite_cache::CacheManager; // ferrite_cache::CacheConfig is in the same module
use ferrite_cli::Args;
use ferrite_core::FeriteApp;
use ferrite_logging::{init, LogConfig, LogLevel}; // Import LogLevel

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Load FerriteConfig first, as it might contain log_file path
    let config = args.handle_config().unwrap_or_else(|e| {
        eprintln!(
            "Configuration error: {}. Run with --generate-config to create \
             one. Error: {}",
            e, e
        );
        std::process::exit(1);
    });

    // Initialize logging using settings from CLI and FerriteConfig
    // Store the guard to keep the file logger alive.
    let _log_guard = init(LogConfig {
        level: args.get_log_level().unwrap_or_else(|err| {
            eprintln!(
                "Warning: Failed to parse log level from CLI: {}. Defaulting to Info.",
                err
            );
            LogLevel::Info
        }),
        enable_tracy: true, // TODO: Consider making this configurable via FerriteConfig
        log_spans: true,    // TODO: Consider making this configurable
        file_path: config.log_file.clone(), // Pass the log file path from FerriteConfig
    });

    // Now that logging is initialized, tracing::* macros can be used.
    tracing::info!("Ferrite application starting...");
    tracing::debug!("Loaded configuration: {:?}", config);
    tracing::debug!("Parsed CLI arguments: {:?}", args);

    // Configure CacheManager using FerriteConfig
    let cache_manager_config = ferrite_cache::CacheConfig {
        max_image_count: config.cache.max_memory_items,
        thread_count: config.cache.worker_threads,
    };
    let cache_manager = Arc::new(CacheManager::new(cache_manager_config));

    if let Some(ref image_path) = args.image_path {
        // Try to load (and thus cache) the initial image.
        // Errors here are not fatal for startup but should be logged.
        if let Err(e) = cache_manager.get_image(image_path.clone()) {
            tracing::warn!(
                "Failed to pre-cache initial image {}: {}",
                image_path.display(),
                e
            );
        } else {
            tracing::info!(
                "Successfully pre-cached initial image: {}",
                image_path.display()
            );
        }
    }

    let mut native_options = eframe::NativeOptions::default();

    let width = config.window.width as f32;
    let height = config.window.height as f32;

    native_options.viewport = ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_decorations(!config.window.borderless)
        .with_title("Ferrite"); // Set a default title

    let result = eframe::run_native(
        "Ferrite", // Internal name for eframe
        native_options,
        Box::new(move |cc| {
            // config and cache_manager are moved into the closure
            let app = Box::new(FeriteApp::new(
                cc,
                args.image_path, // args is cloned or its relevant parts are
                config,
                cache_manager,
            ));
            Ok(app)
        }),
    );

    // _log_guard is dropped here, ensuring logs are flushed.
    result
}
