use std::{path::PathBuf, sync::Arc, thread};

use crate::{
    types::{CacheConfig, CacheHandle, CacheRequest, CacheState, ImageData},
    CacheError,
    CacheResult,
    ImageLoadError,
};
use image::GenericImageView;
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot, RwLock},
};
use tracing::{debug, info, warn};
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
                                    runtime.spawn(async move {
                                        let result = manager.load_and_cache(path).await;
                                        let _ = response_tx.send(result);
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

    // Internal method to handle image retrieval
    async fn get_image_internal(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
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

    pub fn runtime(&self) -> Arc<Runtime> {
        self.runtime_handle.clone()
    }

    pub async fn get_image(
        &self,
        path: PathBuf,
    ) -> CacheResult<Arc<ImageData>> {
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

    async fn lookup_image(&self, path: &PathBuf) -> Option<Arc<ImageData>> {
        let mut state = self.state.write().await;

        if let Some(image) = state.entries.get(path) {
            debug!(path = ?path, "Found image in cache");
            let mut image_data = image.clone();
            image_data.touch();
            let copy_start = std::time::Instant::now();
            let _copied_data = image_data.simulate_copy();
            let copy_duration = copy_start.elapsed();
            debug!(path = ?path, duration = ?copy_duration, "Data copy completed");
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
        let load_start = std::time::Instant::now();
        let path_clone = path.clone();

        let file_size = path_clone
            .metadata()
            .map(|m| m.len())
            .unwrap_or(0);
        debug!(path = ?path, size = file_size, "Loading image from filesystem");

        let rn = self.runtime();
        let image_data = rn.spawn(async move {
            tokio::fs::read(&path_clone).await
        }).await.map_err(|e| {
            warn!(path = ?path, error = ?e, "Failed to spawn image loading task");
            CacheError::ImageLoad {
                path: path.clone(),
                source: ImageLoadError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }
        })?.map_err(|e| {
            warn!(path = ?path, error = ?e, "Failed to read image file");
            CacheError::ImageLoad {
                path: path.clone(),
                source: ImageLoadError::Io(e),
            }
        })?;

        let load_duration = load_start.elapsed();
        debug!(path = ?path, duration = ?load_duration, "File load completed");

        let decode_start = std::time::Instant::now();
        let image = image::load_from_memory(&image_data).map_err(|e| {
            warn!(path = ?path, error = ?e, "Failed to parse image data");
            CacheError::ImageLoad {
                path:   path.clone(),
                source: ImageLoadError::Format(e.to_string()),
            }
        })?;

        let decode_duration = decode_start.elapsed();
        let dimensions = image.dimensions();
        let memory_size = image.as_bytes().len();

        debug!(
            path = ?path,
            duration = ?decode_duration,
            width = dimensions.0,
            height = dimensions.1,
            memory = memory_size,
            "Image decoded"
        );

        let image_data = ImageData::new(image_data, dimensions);
        let mut state = self.state.write().await;

        if state.entries.len() >= self.config.max_image_count {
            if let Some(oldest) = state.lru_list.first().cloned() {
                info!(path = ?oldest, "Evicting least recently used image");
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
