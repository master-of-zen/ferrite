use image::DynamicImage;
use lru::LruCache;
use std::{num::NonZeroUsize, path::PathBuf};

pub struct ImageCache {
    cache: LruCache<PathBuf, DynamicImage>,
}

impl ImageCache {
    pub fn new(size: usize) -> Self {
        Self {
            cache: LruCache::new(
                NonZeroUsize::new(size)
                    .unwrap_or(NonZeroUsize::new(5).unwrap()),
            ),
        }
    }

    pub fn get(&mut self, path: &PathBuf) -> Option<&DynamicImage> {
        self.cache.get(path)
    }

    pub fn put(&mut self, path: PathBuf, image: DynamicImage) {
        self.cache.put(path, image);
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn capacity(&self) -> usize {
        self.cache.cap().get()
    }
}
