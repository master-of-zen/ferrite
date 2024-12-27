use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::info;

use crate::ui::zoom::ZoomHandler;
use ferrite_image::{ImageManager, SupportedFormats};

pub struct NavigationManager {
    directory_images: Vec<PathBuf>,
    current_index:    usize,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self {
            directory_images: Vec::new(), current_index: 0
        }
    }

    pub fn load_current_directory(&mut self, image_path: &Path) -> Option<()> {
        // Get absolute path
        let absolute_path = fs::canonicalize(image_path).ok()?;
        let parent_dir = absolute_path.parent()?;

        info!("Loading images from directory: {}", parent_dir.display());

        // Read directory entries
        let entries = fs::read_dir(parent_dir).ok()?;

        // Collect valid image paths
        self.directory_images = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file()
                    && SupportedFormats::is_supported(path.extension())
                {
                    return Some(path);
                }
                None
            })
            .collect();

        // Sort paths for consistent ordering
        self.directory_images.sort();

        // Find current image index
        self.current_index = self
            .directory_images
            .iter()
            .position(|p| p == &absolute_path)
            .unwrap_or(0);

        info!(
            "Found {} images in directory, current image at index {}",
            self.directory_images.len(),
            self.current_index
        );

        Some(())
    }

    pub fn next_image(&mut self) -> Option<PathBuf> {
        if self.directory_images.is_empty() {
            return None;
        }
        self.current_index =
            (self.current_index + 1) % self.directory_images.len();
        Some(self.directory_images[self.current_index].clone())
    }

    pub fn previous_image(&mut self) -> Option<PathBuf> {
        if self.directory_images.is_empty() {
            return None;
        }
        self.current_index = if self.current_index == 0 {
            self.directory_images.len() - 1
        } else {
            self.current_index - 1
        };
        Some(self.directory_images[self.current_index].clone())
    }

    pub fn handle_keyboard_input(
        &mut self,
        ctx: &eframe::egui::Context,
        image_manager: &mut ImageManager,
        zoom_handler: &mut ZoomHandler,
    ) {
        let next_pressed = ctx.input(|i| {
            i.key_pressed(eframe::egui::Key::ArrowRight)
                || i.key_pressed(eframe::egui::Key::D)
        });
        let prev_pressed = ctx.input(|i| {
            i.key_pressed(eframe::egui::Key::ArrowLeft)
                || i.key_pressed(eframe::egui::Key::A)
        });

        if next_pressed {
            if let Some(next_path) = self.next_image() {
                let _ = image_manager.load_image(next_path);
                zoom_handler.reset_view_position();
            }
        } else if prev_pressed {
            if let Some(prev_path) = self.previous_image() {
                let _ = image_manager.load_image(prev_path);
                zoom_handler.reset_view_position();
            }
        }
    }
}
