use crate::{
    types::{CacheConfig, CacheState, ImageData},
    CacheError,
    CacheResult,
    ImageLoadError,
};
use image::{GenericImageView, ImageFormat};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, warn};

pub struct CacheManager {
    config: CacheConfig,
    state:  Arc<RwLock<CacheState>>,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CacheState::new())),
        }
    }

    pub async fn get_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
        if let Some(image) = self.lookup_image(&path).await {
            return Ok(image);
        }

        let image = self.load_and_cache(path).await?;
        Ok(image)
    }

    async fn lookup_image(&self, path: &PathBuf) -> Option<Arc<ImageData>> {
        let mut state = self.state.write().await;

        if let Some(image) = state.entries.get(path) {
            // Changed to get() instead of get_mut()
            let mut image_data = image.clone();
            image_data.touch();
            state
                .entries
                .insert(path.clone(), image_data.clone()); // Update the touched image
            self.update_lru(path, &mut state).await;
            return Some(Arc::new(image_data));
        }
        None
    }

    async fn load_and_cache(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
        let image_data = tokio::fs::read(&path).await.map_err(|e| {
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Io(e),
            }
        })?;

        let image = image::load_from_memory(&image_data).map_err(|e| {
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Format(e.to_string()),
            }
        })?;

        let dimensions = image.dimensions();
        let image_data = ImageData::new(image_data, dimensions);

        let mut state = self.state.write().await;

        if state.entries.len() >= self.config.max_image_count {
            if let Some(oldest) = state.lru_list.first().cloned() {
                state.entries.remove(&oldest);
                state.lru_list.remove(0);
            }
        }

        state
            .entries
            .insert(path.clone(), image_data.clone());
        state.lru_list.push(path);

        Ok(Arc::new(image_data))
    }

    async fn update_lru(&self, path: &PathBuf, state: &mut CacheState) {
        if let Some(pos) = state.lru_list.iter().position(|p| p == path) {
            state.lru_list.remove(pos);
        }
        state.lru_list.push(path.clone());
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}
