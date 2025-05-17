use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub preload_count: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { preload_count: 2 }
    }
}

impl CacheConfig {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}
