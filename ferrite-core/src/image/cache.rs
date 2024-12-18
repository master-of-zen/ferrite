use image::DynamicImage;
use lru::LruCache;
use std::{num::NonZeroUsize, path::PathBuf};
use tracing::{info, info_span, instrument, Instrument};

pub struct ImageCache {
    cache:  LruCache<PathBuf, DynamicImage>,
    // Track performance metrics
    hits:   usize,
    misses: usize,
}

impl ImageCache {
    #[instrument(skip(size), level = "debug")]
    pub fn new(size: usize) -> Self {
        info!("Initializing image cache with size {}", size);
        Self {
            cache:  LruCache::new(
                NonZeroUsize::new(size)
                    .unwrap_or(NonZeroUsize::new(5).unwrap()),
            ),
            hits:   0,
            misses: 0,
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub fn get(&mut self, path: &PathBuf) -> Option<&DynamicImage> {
        let span = info_span!("cache_lookup", path = ?path);
        let _enter = span.enter();

        let result = self.cache.get(path);
        match result {
            Some(_) => {
                self.hits += 1;
                info!("Cache hit (total hits: {})", self.hits);
            },
            None => {
                self.misses += 1;
                info!("Cache miss (total misses: {})", self.misses);
            },
        }
        result
    }

    #[instrument(skip(self, image), level = "debug")]
    pub fn put(&mut self, path: PathBuf, image: DynamicImage) {
        let span = info_span!("cache_insert", path = ?path);
        let _enter = span.enter();

        info!(
            "Inserting new image into cache (size: {}x{})",
            image.width(),
            image.height()
        );
        self.cache.put(path, image);
    }

    pub fn cache_stats(&self) -> (usize, usize, f64) {
        let total = self.hits + self.misses;
        let hit_rate =
            if total > 0 { self.hits as f64 / total as f64 } else { 0.0 };
        (self.hits, self.misses, hit_rate)
    }

    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    #[instrument(skip(self))]
    pub fn capacity(&self) -> usize {
        self.cache.cap().get()
    }
}
