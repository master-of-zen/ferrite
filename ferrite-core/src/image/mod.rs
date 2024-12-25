use eframe::egui::{self, Context};
use ferrite_cache::CacheHandle;
use ferrite_logging::metrics::PerformanceMetrics;
use image::{DynamicImage, ImageError};
use std::{io, path::PathBuf, sync::Arc};
use thiserror::Error;
use tracing::{info, info_span, instrument, warn};

mod data;
mod formats;

use data::ImageData;
pub use formats::SupportedFormats;

pub struct ImageManager {
    current_image: Option<ImageData>,
    current_path:  Option<PathBuf>,
    cache_manager: Arc<CacheHandle>,
}

impl ImageManager {
    #[instrument(skip_all)]
    pub fn new(cache_manager: Arc<CacheHandle>) -> Self {
        info!("Initializing ImageManager with cache");
        Self {
            current_image: None,
            current_path: None,
            cache_manager,
        }
    }

    // Keep set_image for direct image updates
    pub fn set_image(&mut self, image: DynamicImage) {
        info!("Setting new image directly");
        self.current_image = Some(ImageData::new(image));
    }

    pub fn set_path(&mut self, path: PathBuf) {
        info!("Setting new image path: {}", path.display());
        self.current_path = Some(path);
    }

    pub fn load_image(&mut self, path: PathBuf) -> Result<(), ImageLoadError> {
        let metrics = PerformanceMetrics::new("image_loading", true);

        let result = info_span!("image_loading_process").in_scope(|| {
            // First try to get the image from cache
            match self.cache_manager.get_image(path.clone()) {
                Ok(cached_image) => {
                    // Convert the cached image data into our internal format
                    let image =
                        image::load_from_memory(cached_image.data().as_ref())
                            .map_err(ImageLoadError::ImageError)?;

                    // Use the existing set_image method to update the current
                    // image
                    self.set_image(image);
                    self.set_path(path);

                    let dimensions = self
                        .get_current_dimensions()
                        .expect("Image dimensions should be available");
                    info!(
                        "Successfully loaded image from cache: \
                         dimensions={}x{}",
                        dimensions.0, dimensions.1
                    );

                    Ok(())
                },
                Err(e) => {
                    warn!("Failed to load image from cache: {}", e);
                    Err(ImageLoadError::CacheError(e))
                },
            }
        });

        let duration = metrics.finish();
        info!("Image loading completed in {} ms", duration.as_millis());

        result
    }

    // Add method to preload next/previous images in the background
    pub fn preload_image(&self, path: PathBuf) {
        // Fire and forget preloading
        if let Err(e) = self.cache_manager.cache_image(path) {
            warn!("Failed to preload image: {}", e);
        }
    }

    pub fn get_current_dimensions(&self) -> Option<(u32, u32)> {
        self.current_image
            .as_ref()
            .map(|img| img.dimensions())
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

// Update the error type to include cache errors
#[derive(Error, Debug)]
pub enum ImageLoadError {
    #[error("Failed to access image file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to decode or process image: {0}")]
    ImageError(#[from] ImageError),

    #[error("Invalid image path: {0}")]
    InvalidPath(String),

    #[error("Cache error: {0}")]
    CacheError(#[from] ferrite_cache::CacheError),
}
