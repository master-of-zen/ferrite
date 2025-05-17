use crate::{ConfigError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub preload_count: usize,
    pub max_memory_items: usize,
    pub worker_threads: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { preload_count: 2, max_memory_items: 100, worker_threads: 8 }
    }
}
impl CacheConfig {
    pub fn validate(&self) -> Result<()> {
        if self.worker_threads == 0 {
            return Err(ConfigError::ValidationError(
                "Cache worker_threads must be at least 1.".to_string(),
            ));
        }
        if self.max_memory_items == 0 {
            return Err(ConfigError::ValidationError(
                "Cache max_memory_items must be at least 1.".to_string(),
            ));
        }
        // preload_count can be 0 if no preloading is desired.
        Ok(())
    }
}
