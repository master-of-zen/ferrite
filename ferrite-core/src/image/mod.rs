use eframe::egui::{self, Context};
use ferrite_logging::metrics::PerformanceMetrics;
use std::{fs, path::PathBuf, time::Instant};
use tracing::{info, info_span, instrument, warn, Instrument};

mod data;

use data::ImageData;

pub struct ImageManager {
    current_image: Option<ImageData>,
    current_path:  Option<PathBuf>,
}

use image::ImageError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageLoadError {
    #[error("Failed to access image file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to decode or process image: {0}")]
    ImageError(#[from] ImageError),

    #[error("Invalid image path: {0}")]
    InvalidPath(String),
}

impl ImageManager {
    #[instrument(skip_all)]
    pub fn new() -> Self {
        info!("Initializing ImageManager");
        Self {
            current_image: None, current_path: None
        }
    }

    pub fn load_image(&mut self, path: PathBuf) -> Result<(), ImageLoadError> {
        let metrics = PerformanceMetrics::new("image_loading", true);

        let result = info_span!("image_loading_process").in_scope(|| {
            // Try to get absolute path
            let absolute_path = fs::canonicalize(&path).map_err(|e| {
                warn!("Failed to resolve path: {}", e);
                ImageLoadError::IoError(e)
            })?;

            if !absolute_path.exists() {
                return Err(ImageLoadError::InvalidPath(format!(
                    "Path does not exist: {}",
                    absolute_path.display()
                )));
            }

            info!("Loading image from disk: {}", absolute_path.display());
            match image::open(&absolute_path) {
                Ok(img) => {
                    let dimensions = (img.width(), img.height());
                    info!(
                        "Successfully loaded image: dimensions={}x{}",
                        dimensions.0, dimensions.1
                    );

                    self.current_image = Some(ImageData::new(img));
                    self.current_path = Some(absolute_path);
                    Ok(())
                },
                Err(e) => {
                    warn!("Failed to load image: {}", e);
                    Err(ImageLoadError::ImageError(e))
                },
            }
        });

        // Record performance metrics regardless of the result
        let duration = metrics.finish();
        info!("Image loading completed in {} ms", duration.as_millis());

        result
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
