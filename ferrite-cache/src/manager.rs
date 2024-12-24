use crate::{
    types::{CacheConfig, CacheState, ImageData},
    CacheError,
    CacheResult,
    ImageLoadError,
};
use image::GenericImageView;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub struct CacheManager {
    config: CacheConfig,
    state:  Arc<RwLock<CacheState>>,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        info!(
            max_images = config.max_image_count,
            "Initializing cache manager"
        );
        Self {
            config,
            state: Arc::new(RwLock::new(CacheState::new())),
        }
    }

    pub async fn get_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
        debug!(path = ?path, "Image requested from cache");

        if let Some(image) = self.lookup_image(&path).await {
            info!(path = ?path, "Cache hit");
            return Ok(image);
        }

        info!(path = ?path, "Cache miss, loading from disk");
        let image = self.load_and_cache(path).await?;
        Ok(image)
    }

    async fn lookup_image(&self, path: &PathBuf) -> Option<Arc<ImageData>> {
        let mut state = self.state.write().await;

        if let Some(image) = state.entries.get(path) {
            debug!(path = ?path, "Found image in cache");
            let mut image_data = image.clone();
            image_data.touch();
            state
                .entries
                .insert(path.clone(), image_data.clone());
            self.update_lru(path, &mut state).await;
            return Some(Arc::new(image_data));
        }
        debug!(path = ?path, "Image not found in cache");
        None
    }

    async fn load_and_cache(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
        debug!(path = ?path, "Loading image from filesystem");

        let image_data = tokio::fs::read(&path).await.map_err(|e| {
            warn!(
                path = ?path,
                error = ?e,
                "Failed to read image file"
            );
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Io(e),
            }
        })?;

        let image = image::load_from_memory(&image_data).map_err(|e| {
            warn!(
                path = ?path,
                error = ?e,
                "Failed to parse image data"
            );
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Format(e.to_string()),
            }
        })?;

        let dimensions = image.dimensions();
        info!(
            path = ?path,
            width = dimensions.0,
            height = dimensions.1,
            "Successfully loaded image"
        );

        let image_data = ImageData::new(image_data, dimensions);
        let mut state = self.state.write().await;

        if state.entries.len() >= self.config.max_image_count {
            if let Some(oldest) = state.lru_list.first().cloned() {
                info!(
                    path = ?oldest,
                    "Evicting least recently used image"
                );
                state.entries.remove(&oldest);
                state.lru_list.remove(0);
            }
        }

        debug!(
            path = ?path,
            cache_size = state.entries.len(),
            "Adding image to cache"
        );

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
        debug!(
            path = ?path,
            list_size = state.lru_list.len(),
            "Updated LRU list"
        );
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}
