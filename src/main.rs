use std::{path::PathBuf, sync::Arc};

use eframe::{egui::ViewportBuilder, Error};
use ferrite::{
    cache::CacheManager,
    cli::Args,
    config::FerriteConfig,
    core::FeriteApp,
    logging::{init, LogConfig, LogLevel},
};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let config = args.handle_config().unwrap_or_else(|e| {
        eprintln!(
            "Configuration error: {}. Run with --generate-config to create \
             one. Error: {}",
            e, e
        );
        std::process::exit(1);
    });

    // Determine the actual log file path
    let mut final_log_path: Option<PathBuf> = None;

    if let Some(log_file_in_config) = &config.log_file {
        if log_file_in_config.is_absolute() {
            // If the path in config is absolute, use it directly.
            final_log_path = Some(log_file_in_config.clone());
        } else {
            // It's a relative path, resolve it against the config file's
            // directory.
            match FerriteConfig::resolve_config_path() {
                Ok(config_file_actual_path) => {
                    if let Some(config_dir) = config_file_actual_path.parent() {
                        final_log_path =
                            Some(config_dir.join(log_file_in_config));
                    } else {
                        // config_file_actual_path has no parent (e.g.,
                        // "config.toml" in CWD)
                        // Log path is relative to CWD.
                        final_log_path = Some(log_file_in_config.clone());
                        tracing::debug!(
                            "Config file path {} has no parent. Log path {} \
                             will be relative to CWD.",
                            config_file_actual_path.display(),
                            log_file_in_config.display()
                        );
                    }
                },
                Err(e) => {
                    // Could not resolve config directory. Log path will be
                    // relative to CWD.
                    final_log_path = Some(log_file_in_config.clone());
                    tracing::warn!(
                        "Could not resolve config directory to make log_file \
                         path absolute: {}. Log path {} will be relative to \
                         CWD.",
                        e,
                        log_file_in_config.display()
                    );
                },
            }
        }
    }
    // If config.log_file was None, final_log_path remains None.

    let _log_guard = init(LogConfig {
        level:        args.get_log_level().unwrap_or_else(|err| {
            eprintln!(
                "Warning: Failed to parse log level from CLI: {}. Defaulting \
                 to Info.",
                err
            );
            LogLevel::Info
        }),
        enable_tracy: true,
        log_spans:    true,
        file_path:    final_log_path.clone(), /* Pass the resolved or original absolute/None path */
    });

    tracing::info!("Ferrite application starting...");
    tracing::debug!("Loaded configuration: {:?}", config);

    if let Some(p) = &final_log_path {
        tracing::info!("Logging to file: {}", p.display());
    } else if config.log_file.is_some() {
        // This case implies resolution might have failed but a path was
        // configured
        tracing::info!(
            "Attempting to log to file (path might be CWD relative if config \
             dir resolution failed): {}",
            config.log_file.as_ref().unwrap().display()
        );
    } else {
        tracing::info!("File logging is disabled (no path configured).");
    }

    tracing::debug!("Parsed CLI arguments: {:?}", args);

    let cache_manager_config = ferrite::cache::CacheConfig {
        max_image_count: config.cache.max_memory_items,
        thread_count:    config.cache.worker_threads,
    };
    let cache_manager = Arc::new(CacheManager::new(cache_manager_config));

    if let Some(ref image_path) = args.image_path {
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
        .with_min_inner_size([640.0, 480.0])
        .with_decorations(!config.window.borderless)
        .with_title("Ferrite");

    let result = eframe::run_native(
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
    );

    result
}
