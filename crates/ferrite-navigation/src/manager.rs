use crate::error::{NavigationError, Result};
use ferrite_image::SupportedFormats;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::{info, instrument};

pub struct NavigationManager {
    directory_images: Vec<PathBuf>,
    current_index: usize,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self { directory_images: Vec::new(), current_index: 0 }
    }

    #[instrument(skip(self, image_path), fields(path = ?image_path))]
    pub fn load_current_directory(&mut self, image_path: &Path) -> Result<()> {
        let absolute_path = fs::canonicalize(image_path)
            .map_err(NavigationError::DirectoryAccess)?;

        let parent_dir = absolute_path.parent().ok_or_else(|| {
            NavigationError::InvalidPath(image_path.to_path_buf())
        })?;

        info!("Loading images from directory: {}", parent_dir.display());

        // Collect valid image paths
        self.directory_images = fs::read_dir(parent_dir)
            .map_err(NavigationError::DirectoryAccess)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_file()
                        && SupportedFormats::is_supported(path.extension())
                    {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .collect();

        self.directory_images.sort();

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

        Ok(())
    }

    pub fn get_nearby_paths(
        &self,
        count: usize,
    ) -> (Vec<PathBuf>, Vec<PathBuf>) {
        if self.directory_images.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let total_images = self.directory_images.len();
        let mut next_paths = Vec::with_capacity(count);
        let mut prev_paths = Vec::with_capacity(count);

        for i in 1..=count {
            // Get next images circularly
            let next_index = (self.current_index + i) % total_images;
            if next_index != self.current_index {
                next_paths.push(self.directory_images[next_index].clone());
            }

            // Get previous images circularly
            let prev_index = if i <= self.current_index {
                self.current_index - i
            } else {
                total_images - (i - self.current_index)
            };
            if prev_index < total_images && prev_index != self.current_index {
                prev_paths.push(self.directory_images[prev_index].clone());
            }
        }

        (prev_paths, next_paths)
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
}
