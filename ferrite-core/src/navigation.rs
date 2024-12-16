use eframe::egui::{Context, Key};
use std::{fs, path::PathBuf};
use tracing::info;

use crate::image::ImageManager;

pub struct NavigationManager {
    directory_images: Vec<PathBuf>,
    current_index: usize,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self {
            directory_images: Vec::new(),
            current_index: 0,
        }
    }

    pub fn handle_keyboard_input(&mut self, ctx: &Context, image_manager: &mut ImageManager) {
        let next_pressed = ctx.input(|i| i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::D));
        let prev_pressed = ctx.input(|i| i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::A));

        if next_pressed {
            self.next_image(image_manager);
        } else if prev_pressed {
            self.previous_image(image_manager);
        }
    }

    fn next_image(&mut self, image_manager: &mut ImageManager) {
        if !self.directory_images.is_empty() {
            self.current_index = (self.current_index + 1) % self.directory_images.len();
            image_manager.load_image(self.directory_images[self.current_index].clone());
        }
    }

    fn previous_image(&mut self, image_manager: &mut ImageManager) {
        if !self.directory_images.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.directory_images.len() - 1
            } else {
                self.current_index - 1
            };
            image_manager.load_image(self.directory_images[self.current_index].clone());
        }
    }

    pub fn load_directory_images(&mut self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            if let Ok(entries) = fs::read_dir(parent) {
                self.directory_images = entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(extension) = path.extension() {
                                if matches!(
                                    extension.to_str().map(|s| s.to_lowercase()),
                                    Some(ext) if ["jpg", "jpeg", "png", "gif", "bmp"].contains(&ext.as_str())
                                ) {
                                    return Some(path);
                                }
                            }
                        }
                        None
                    })
                    .collect();

                self.directory_images.sort();
                info!("Found {} images in directory", self.directory_images.len());
            }
        }
    }
}
