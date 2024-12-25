use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct ImageData {
    data:        Arc<Vec<u8>>,
    dimensions:  (u32, u32),
    accessed_at: Instant,
}

impl ImageData {
    pub fn new(data: Vec<u8>, dimensions: (u32, u32)) -> Self {
        debug!(
            width = dimensions.0,
            height = dimensions.1,
            size_bytes = data.len(),
            "Creating new ImageData instance"
        );

        Self {
            data: Arc::new(data),
            dimensions,
            accessed_at: Instant::now(),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn data(&self) -> Arc<Vec<u8>> {
        self.data.clone()
    }

    pub fn touch(&mut self) {
        let previous = self.accessed_at;
        self.accessed_at = Instant::now();

        debug!(
            last_access = ?previous,
            new_access = ?self.accessed_at,
            "Updated image access time"
        );
    }

    pub fn simulate_copy(&self) -> Vec<u8> {
        // Simulate copying the full decoded image data
        self.data.to_vec()
    }
}

#[derive(Clone)]
pub struct CacheConfig {
    pub max_image_count: usize,
    pub thread_count:    usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_image_count: 100, thread_count: 4
        }
    }
}

pub(crate) struct CacheState {
    pub entries:  HashMap<PathBuf, ImageData>,
    pub lru_list: Vec<PathBuf>,
}

impl CacheState {
    pub fn new() -> Self {
        debug!("Initializing new cache state");

        Self {
            entries: HashMap::new(), lru_list: Vec::new()
        }
    }
}
