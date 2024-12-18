use cache::ImageCache;
use data::ImageData;
use eframe::egui::{self, Context};
use ferrite_logging::metrics::PerformanceMetrics;
use std::path::PathBuf;
use tracing::{info, info_span, instrument, warn, Instrument};

mod cache;
mod data;

pub struct ImageManager {
    cache:         ImageCache,
    current_image: Option<ImageData>,
    current_path:  Option<PathBuf>,
}

impl ImageManager {
    #[instrument(skip_all)]
    pub fn new() -> Self {
        info!("Initializing ImageManager");
        Self {
            cache:         ImageCache::new(5),
            current_image: None,
            current_path:  None,
        }
    }

    #[instrument(skip(self), fields(path = ?path))]
    pub fn load_image(&mut self, path: PathBuf) {
        // Create a performance measurement context
        let metrics = PerformanceMetrics::new("image_loading", true);

        let result = info_span!("image_loading_process").in_scope(|| {
            if let Some(img) = self.cache.get(&path) {
                info!("Loading image from cache");
                self.current_image = Some(ImageData::new(img.clone()));
                self.current_path = Some(path);
                return Ok(());
            }

            info!("Loading image from disk");
            match image::open(&path) {
                Ok(img) => {
                    let dimensions = (img.width(), img.height());
                    info!(
                        "Successfully loaded image: dimensions={}x{}",
                        dimensions.0, dimensions.1
                    );

                    // Cache the loaded image
                    info_span!("caching_image").in_scope(|| {
                        self.cache.put(path.clone(), img.clone());
                    });

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
            // Get cache statistics
            let (hits, misses, hit_rate) = self.cache.cache_stats();

            ui.label(format!(
                "Cache size: {}/{}",
                self.cache.len(),
                self.cache.capacity()
            ));
            ui.label(format!("Cache hits: {}", hits));
            ui.label(format!("Cache misses: {}", misses));
            ui.label(format!("Cache hit rate: {:.1}%", hit_rate * 100.0));

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
