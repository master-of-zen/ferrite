use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};
use tracing::{debug, info};

use crate::CacheResult;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum CacheRequest {
    CacheImage {
        path:        PathBuf,
        response_tx: oneshot::Sender<CacheResult<()>>, // Changed return type
    },
    GetImage {
        path:        PathBuf,
        response_tx: oneshot::Sender<CacheResult<Arc<ImageData>>>,
    },
}

// Structure to handle communication with the cache manager
pub struct CacheHandle {
    request_tx: tokio::sync::mpsc::UnboundedSender<CacheRequest>,
}

impl CacheHandle {
    pub fn new(
        request_tx: tokio::sync::mpsc::UnboundedSender<CacheRequest>,
    ) -> Self {
        Self {
            request_tx,
        }
    }

    // Public API for requesting an image - this hides the channel communication
    pub fn get_image(&self, path: PathBuf) -> CacheResult<Arc<ImageData>> {
        // Create a one-shot channel for the response
        let (response_tx, response_rx) = oneshot::channel();

        // Send the request through the unbounded channel
        self.request_tx
            .send(CacheRequest::GetImage {
                path,
                response_tx,
            })
            .map_err(|_| {
                crate::CacheError::Config(
                    "Cache manager is shutdown".to_string(),
                )
            })?;

        // Wait for and return the response
        response_rx.blocking_recv().map_err(|_| {
            crate::CacheError::Config(
                "Cache manager stopped responding".to_string(),
            )
        })?
    }

    pub fn cache_image(&self, path: PathBuf) -> CacheResult<()> {
        let (response_tx, response_rx) = oneshot::channel();

        self.request_tx
            .send(CacheRequest::CacheImage {
                path,
                response_tx,
            })
            .map_err(|_| {
                crate::CacheError::Config(
                    "Cache manager is shutdown".to_string(),
                )
            })?;

        // Just wait for acknowledgment
        response_rx.blocking_recv().map_err(|_| {
            crate::CacheError::Config(
                "Cache manager stopped responding".to_string(),
            )
        })?
    }
}

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
