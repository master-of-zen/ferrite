use eframe::egui::{self, Context};
use ferrite_logging::metrics::PerformanceMetrics;
use std::{path::PathBuf, time::Instant};
use tracing::{info, info_span, instrument, warn, Instrument};

mod data;

use data::ImageData;

pub struct ImageManager {
    current_image: Option<ImageData>,
    current_path:  Option<PathBuf>,
}

impl ImageManager {
    #[instrument(skip_all)]
    pub fn new() -> Self {
        info!("Initializing ImageManager");
        Self {
            current_image: None, current_path: None
        }
    }

    #[instrument(skip(self), fields(path = ?path, start_time = ?Instant::now()))]
    pub fn load_image(&mut self, path: PathBuf) {
        let operation_start = Instant::now();
        // Create a performance measurement context
        let metrics = PerformanceMetrics::new("image_loading", true);

        let result = info_span!("image_loading_process").in_scope(|| {
            info!("Loading image from disk");
            match image::open(&path) {
                Ok(img) => {
                    let dimensions = (img.width(), img.height());
                    info!(
                        "Successfully loaded image: dimensions={}x{}",
                        dimensions.0, dimensions.1
                    );

                    self.current_image = Some(ImageData::new(img));
                    self.current_path = Some(path);
                    Ok(())
                },
                Err(e) => Err(e),
            }
        });

        if let Err(e) = result {
            warn!("Failed to load image: {}", e);
        }

        // Record the performance metrics
        let duration = metrics.finish();
        info!("Image loading completed in {} ms", duration.as_millis());
    }

    #[instrument(skip(self, ctx))]
    pub fn show_performance_window(&self, ctx: &Context) {
        egui::Window::new("Performance Metrics").show(ctx, |ui| {
            ui.heading("Image Information");

            if let Some(ref img) = self.current_image {
                let dims = img.dimensions();
                ui.label(format!(
                    "Current image dimensions: {}x{}",
                    dims.0, dims.1
                ));
            }

            if let Some(path) = &self.current_path {
                ui.label(format!(
                    "Current image: {:?}",
                    path.file_name().unwrap_or_default()
                ));
            }
        });
    }

    pub fn current_image(&mut self) -> Option<&mut ImageData> {
        self.current_image.as_mut()
    }
}
