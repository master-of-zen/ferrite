use crate::{
    types::{CacheConfig, CacheState, ImageData},
    CacheError,
    CacheResult,
    ImageLoadError,
};
use image::GenericImageView;
use std::{path::PathBuf, sync::Arc};
use tokio::{runtime::Runtime, sync::RwLock};
use tracing::{debug, info, warn};

pub struct CacheManager {
    config:  CacheConfig,
    state:   Arc<RwLock<CacheState>>,
    runtime: Arc<Runtime>,
}

impl CacheManager {
    pub fn new(config: CacheConfig, runtime: Arc<Runtime>) -> Self {
        info!(
            max_images = config.max_image_count,
            "Initializing cache manager"
        );
        Self {
            config,
            state: Arc::new(RwLock::new(CacheState::new())),
            runtime,
        }
    }

    pub async fn get_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
        let start_time = std::time::Instant::now();
        println!("🔍 Attempting to get image: {:?}", path);

        if let Some(image) = self.lookup_image(&path).await {
            let duration = start_time.elapsed();
            println!("✅ Cache hit for: {:?}", path);
            println!("⏱️ Total time (cache hit): {:?}", duration);
            return Ok(image);
        }

        println!("Cache miss, loading from disk");
        let image = self.load_and_cache(path).await?;
        let duration = start_time.elapsed();
        println!("⏱️ Total time (cache miss): {:?}", duration);
        Ok(image)
    }

    async fn lookup_image(&self, path: &PathBuf) -> Option<Arc<ImageData>> {
        let mut state = self.state.write().await;

        if let Some(image) = state.entries.get(path) {
            debug!(path = ?path, "Found image in cache");
            let mut image_data = image.clone();
            image_data.touch();
            let copy_start = std::time::Instant::now();
            let _copied_data = image_data.simulate_copy();
            let copy_duration = copy_start.elapsed();
            println!("⏱️ Data copy time: {:?}", copy_duration);
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

        let path_clone = path.clone();
        println!("📥 Loading image from disk: {:?}", path);
        println!(
            "   Raw file size: {} bytes",
            path_clone
                .metadata()
                .map(|m| m.len())
                .unwrap_or(0)
        );
        let image_data = self
            .runtime
            .spawn(async move { tokio::fs::read(&path_clone).await })
            .await
            .map_err(|e| {
                warn!(
                    path = ?path,
                    error = ?e,
                    "Failed to spawn image loading task"
                );
                CacheError::ImageLoad {
                    path:   path.clone(),
                    source: ImageLoadError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e,
                    )),
                }
            })?
            .map_err(|e| {
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
        let memory_size = image.as_bytes().len();
        println!("📊 Image decoded:");
        println!("   Dimensions: {}x{}", dimensions.0, dimensions.1);
        println!("   Memory size: {} bytes", memory_size);
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

    pub async fn get_total_memory_usage(&self) -> CacheResult<usize> {
        let state = self.state.read().await;
        Ok(state
            .entries
            .values()
            .map(|img| img.data().len())
            .sum())
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
