use std::{path::PathBuf, sync::Arc, thread};

use crate::{
    types::{CacheConfig, CacheHandle, CacheRequest, CacheState},
    CacheError,
    CacheResult,
    ImageLoadError,
};
use image::{DynamicImage, GenericImageView};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot, RwLock},
};
use tracing::{debug, info};
pub struct CacheManager {
    config:         CacheConfig,
    state:          Arc<RwLock<CacheState>>,
    runtime_handle: Arc<Runtime>,
    _shutdown_tx:   oneshot::Sender<()>,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> CacheHandle {
        let (request_tx, mut request_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let state = Arc::new(RwLock::new(CacheState::new()));

        thread::spawn(move || {
            let runtime = Arc::new(
                tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(config.thread_count)
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime"),
            );

            let manager = Arc::new(Self {
                config,
                state: state.clone(),
                runtime_handle: runtime.clone(),
                _shutdown_tx: shutdown_tx,
            });

            runtime.block_on(async {
                let shutdown_future = shutdown_rx;
                tokio::pin!(shutdown_future);

                loop {
                    tokio::select! {
                        _ = &mut shutdown_future => {
                            debug!("Received shutdown signal");
                            break;
                        }
                        Some(request) = request_rx.recv() => {
                            let manager = manager.clone();
                            match request {
                                CacheRequest::GetImage { path, response_tx } => {
                                    runtime.spawn(async move {
                                        let result = manager.get_image_internal(path).await;
                                        let _ = response_tx.send(result);
                                    });
                                }
                                CacheRequest::CacheImage { path, response_tx } => {
        let manager = Arc::clone(&manager);
        runtime.spawn(async move {
            manager.handle_cache_request(path, response_tx).await;
        });
    }
                            }
                        }
                        else => break,
                    }
                }
                debug!("Cache manager event loop terminated");
            });
        });

        CacheHandle::new(request_tx)
    }

    async fn handle_cache_request(
        &self,
        path: PathBuf,
        response_tx: oneshot::Sender<CacheResult<()>>,
    ) {
        // Clone what we need before spawning
        let state = Arc::clone(&self.state);
        let config = self.config.clone();
        let runtime = self.runtime();

        runtime.spawn(async move {
            let file_size = tokio::fs::metadata(&path).await.map_err(|e| {
                CacheError::ImageLoad {
                    path:   path.clone(),
                    source: ImageLoadError::Io(e),
                }
            })?;

            // Respond immediately with acknowledgment
            let _ = response_tx.send(Ok(()));

            // Continue loading in background
            let image_data = tokio::fs::read(&path).await.map_err(|e| {
                CacheError::ImageLoad {
                    path:   path.clone(),
                    source: ImageLoadError::Io(e),
                }
            })?;

            let decoded_image =
                image::load_from_memory(&image_data).map_err(|e| {
                    CacheError::ImageLoad {
                        path:   path.clone(),
                        source: ImageLoadError::Format(e.to_string()),
                    }
                })?;
            // Update cache state
            let mut state = state.write().await;

            // Check if we need to evict images to make space
            if state.entries.len() >= config.max_image_count {
                if let Some(oldest_path) = state.lru_list.first().cloned() {
                    info!(
                        path = ?oldest_path,
                        "Evicting least recently used image"
                    );
                    state.entries.remove(&oldest_path);
                    state.lru_list.remove(0);
                }
            }

            // Update LRU list
            if let Some(pos) = state.lru_list.iter().position(|p| p == &path) {
                state.lru_list.remove(pos);
            }
            state.lru_list.push(path.clone());

            // Store the image data
            state.entries.insert(path.clone(), decoded_image);

            debug!(
                path = ?path,
                cache_size = state.entries.len(),
                "Image cached successfully"
            );

            Ok::<(), CacheError>(())
        });
    }

    async fn get_image_internal(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<DynamicImage>> {
        let start_time = std::time::Instant::now();
        debug!(path = ?path, "Image requested from cache");

        if let Some(image) = self.lookup_image(&path).await {
            let duration = start_time.elapsed();
            debug!(path = ?path, duration = ?duration, "Cache hit");
            return Ok(image);
        }

        debug!(path = ?path, "Cache miss, loading from disk");
        let image = self.load_and_cache(path.clone()).await?;
        let duration = start_time.elapsed();
        debug!(path = ?path, duration = ?duration, "Total cache miss time");
        Ok(image)
    }

    pub async fn cache_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<DynamicImage>> {
        let file_size = tokio::fs::metadata(&path)
            .await
            .map_err(|e| CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Io(e),
            })?
            .len();

        debug!(
            path = ?path,
            size = file_size,
            "Loading image from filesystem"
        );

        // Read the file contents using tokio's async file IO
        let image_data = tokio::fs::read(&path).await.map_err(|e| {
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Io(e),
            }
        })?;

        let image_data = image::load_from_memory(&image_data).unwrap();

        let mut state = self.state.write().await;

        if state.entries.len() >= self.config.max_image_count {
            if let Some(oldest_path) = state.lru_list.first().cloned() {
                info!(
                    path = ?oldest_path,
                    "Evicting least recently used image"
                );
                state.entries.remove(&oldest_path);
                state.lru_list.remove(0);
            }
        }

        // Update LRU list - remove if exists and add to end
        if let Some(pos) = state.lru_list.iter().position(|p| p == &path) {
            state.lru_list.remove(pos);
        }
        state.lru_list.push(path.clone());

        // Store the image data
        let image_data = Arc::new(image_data);
        state
            .entries
            .insert(path.clone(), (*image_data).clone());

        debug!(
            path = ?path,
            cache_size = state.entries.len(),
            "Image cached successfully"
        );

        Ok(image_data)
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        self.runtime_handle.clone()
    }

    pub async fn get_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<DynamicImage>> {
        let start_time = std::time::Instant::now();
        debug!(path = ?path, "Image requested from cache");

        if let Some(image) = self.lookup_image(&path).await {
            let duration = start_time.elapsed();
            debug!(path = ?path, duration = ?duration, "Cache hit");
            return Ok(image);
        }

        debug!(path = ?path, "Cache miss, loading from disk");
        let image = self.load_and_cache(path.clone()).await?;
        let duration = start_time.elapsed();
        debug!(path = ?path, duration = ?duration, "Total cache miss time");
        Ok(image)
    }

    // Shit code
    async fn lookup_image(&self, path: &PathBuf) -> Option<Arc<DynamicImage>> {
        let mut state = self.state.write().await;

        if let Some(image) = state.entries.get(path) {
            debug!(path = ?path, "Found image in cache");
            return Some(Arc::new(image.clone()));
        }
        self.update_lru(path, &mut state).await;

        debug!(path = ?path, "Image not found in cache");
        None
    }

    async fn load_and_cache(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<DynamicImage>> {
        let load_start = std::time::Instant::now();

        // Read file data
        let file_data = tokio::fs::read(&path).await.map_err(|e| {
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Io(e),
            }
        })?;

        // Decode image immediately
        let decoded_image =
            image::load_from_memory(&file_data).map_err(|e| {
                CacheError::ImageLoad {
                    path:   path.clone(),
                    source: ImageLoadError::Format(e.to_string()),
                }
            })?;

        let dimensions = decoded_image.dimensions();

        // Update cache state
        let mut state = self.state.write().await;

        // Handle cache eviction if needed
        if state.entries.len() >= self.config.max_image_count {
            if let Some(oldest_path) = state.lru_list.first().cloned() {
                info!(path = ?oldest_path, "Evicting least recently used image");
                state.entries.remove(&oldest_path);
                state.lru_list.remove(0);
            }
        }

        debug!(
            path = ?path,
            width = dimensions.0,
            height = dimensions.1,
            load_duration = ?load_start.elapsed(),
            "Image loaded and decoded"
        );

        // Store the decoded image
        let image_data = Arc::new(decoded_image.clone());
        state.entries.insert(path.clone(), decoded_image);
        state.lru_list.push(path);

        Ok(image_data)
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
