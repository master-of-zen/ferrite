use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};

#[derive(Debug, Clone)]
pub struct ImageData {
    data:        Arc<Vec<u8>>,
    dimensions:  (u32, u32),
    accessed_at: Instant,
}

impl ImageData {
    pub fn new(data: Vec<u8>, dimensions: (u32, u32)) -> Self {
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
        self.accessed_at = Instant::now();
    }
}

pub struct CacheConfig {
    pub max_image_count: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_image_count: 100
        }
    }
}

pub(crate) struct CacheState {
    pub entries:  HashMap<PathBuf, ImageData>,
    pub lru_list: Vec<PathBuf>,
}

impl CacheState {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(), lru_list: Vec::new()
        }
    }
}
