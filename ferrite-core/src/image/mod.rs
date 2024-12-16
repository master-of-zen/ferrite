mod cache;
mod data;

use eframe::egui::{self, Context};
use image::DynamicImage;
use std::path::PathBuf;
use tracing::info;

pub(crate) use cache::ImageCache;
pub(crate) use data::ImageData;

pub struct ImageManager {
    cache: ImageCache,
    current_image: Option<ImageData>,
    current_path: Option<PathBuf>,
}

impl ImageManager {
    pub fn new(cache_size: usize) -> Self {
        Self {
            cache: ImageCache::new(cache_size),
            current_image: None,
            current_path: None,
        }
    }

    pub fn load_image(&mut self, path: PathBuf) {
        info!("Loading image: {:?}", path);

        if let Some(img) = self.cache.get(&path) {
            self.current_image = Some(ImageData::new(img.clone()));
            self.current_path = Some(path);
            return;
        }

        match image::open(&path) {
            Ok(img) => {
                self.cache.put(path.clone(), img.clone());
                self.current_image = Some(ImageData::new(img));
                self.current_path = Some(path);
            }
            Err(e) => {
                tracing::warn!("Failed to load image: {}", e);
            }
        }
    }

    pub fn show_performance_window(&self, ctx: &Context) {
        egui::Window::new("Performance").show(ctx, |ui| {
            ui.label(format!(
                "Cache size: {}/{}",
                self.cache.len(),
                self.cache.capacity()
            ));
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
