use crate::{
    cache::{CacheHandle, CacheResult},
    logging::metrics::PerformanceMetrics,
};
use eframe::egui::{self, Context};
use image::{DynamicImage, GenericImageView};
use std::{path::PathBuf, sync::Arc};
use tracing::{info, info_span, instrument, warn};

use super::error::{ImageError, Result};

pub struct ImageManager {
    pub current_image: Option<Arc<DynamicImage>>,
    pub texture:       Option<egui::TextureHandle>,
    pub current_path:  Option<PathBuf>,
    pub cache_manager: Arc<CacheHandle>,
}

impl ImageManager {
    #[instrument(skip_all)]
    pub fn new(cache_manager: Arc<CacheHandle>) -> Self {
        info!("Initializing ImageManager with cache");
        Self {
            current_image: None,
            texture: None,
            current_path: None,
            cache_manager,
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        info!("Setting new image path: {}", path.display());
        self.current_path = Some(path);
    }

    #[instrument(skip(self, path), fields(path = ?path))]
    pub fn load_image(&mut self, path: PathBuf) -> Result<()> {
        let metrics = PerformanceMetrics::new("image_loading", true);

        let result = info_span!("image_loading_process").in_scope(|| {
            let get_image: CacheResult<Arc<DynamicImage>> =
                self.cache_manager.get_image(path.clone());

            if let Ok(image_data) = get_image {
                let dimensions = image_data.dimensions();
                info!("Setting new image and clearing texture");

                self.texture = None;

                self.current_image = Some(image_data);
                self.current_path = Some(path);

                info!(
                    "Successfully loaded image from cache: dimensions={}x{}",
                    dimensions.0, dimensions.1
                );
                Ok(())
            } else {
                Err(ImageError::CacheError(get_image.unwrap_err()))
            }
        });

        let duration = metrics.finish();
        info!("Image loading completed in {} µs", duration.as_micros());

        result
    }

    pub fn preload_image(&self, path: PathBuf) {
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

    /// Delete the current image file by moving it to trash
    ///
    /// Returns the path that was deleted if successful
    #[instrument(skip(self))]
    pub fn delete_current_file(
        &mut self,
    ) -> super::error::Result<Option<PathBuf>> {
        use super::operations::FileOperations;

        if let Some(current_path) = self.current_path.take() {
            // Clear current image and texture since we're deleting the file
            self.current_image = None;
            self.texture = None;

            // Move to trash
            FileOperations::delete_file(&current_path).map_err(|e| {
                super::error::ImageError::Other(format!("Delete failed: {}", e))
            })?;

            info!("Successfully deleted file: {}", current_path.display());
            Ok(Some(current_path))
        } else {
            warn!("No current file to delete");
            Ok(None)
        }
    }
}
