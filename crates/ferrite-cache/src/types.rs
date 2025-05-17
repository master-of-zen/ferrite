use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tracing::debug;

use crate::{CacheError, CacheResult};
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum CacheRequest {
    CacheImage {
        path: PathBuf,
        response_tx: oneshot::Sender<CacheResult<()>>, // Changed return type
    },
    GetImage {
        path: PathBuf,
        response_tx: oneshot::Sender<CacheResult<Arc<DynamicImage>>>,
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
        Self { request_tx }
    }

    // Public API for requesting an image - this hides the channel communication
    pub fn get_image(&self, path: PathBuf) -> CacheResult<Arc<DynamicImage>> {
        // Create a one-shot channel for the response
        let (response_tx, response_rx) = oneshot::channel();

        // Send the request through the unbounded channel
        self.request_tx
            .send(CacheRequest::GetImage { path, response_tx })
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
        let (response_tx, _response_rx) = oneshot::channel();

        self.request_tx
            .send(CacheRequest::CacheImage { path, response_tx })
            .map_err(|_| {
                crate::CacheError::Config(
                    "Cache manager is shutdown".to_string(),
                )
            })?;

        Ok(())
    }
}

use image::DynamicImage;

#[derive(Clone, Debug)]
pub struct CacheConfig {
    pub max_image_count: usize,
    pub thread_count: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { max_image_count: 100, thread_count: 4 }
    }
}

pub(crate) struct CacheState {
    pub entries: HashMap<PathBuf, DynamicImage>,
    pub lru_list: Vec<PathBuf>,
}

impl CacheState {
    pub fn new() -> Self {
        debug!("Initializing new cache state");

        Self { entries: HashMap::new(), lru_list: Vec::new() }
    }
}

impl CacheConfig {
    const MAX_IMAGE_COUNT: usize = 1000;
    const MAX_THREAD_COUNT: usize = 32;
    const MIN_IMAGE_COUNT: usize = 10;
    const MIN_THREAD_COUNT: usize = 1;

    pub fn validate(&self) -> CacheResult<()> {
        if self.thread_count < Self::MIN_THREAD_COUNT
            || self.thread_count > Self::MAX_THREAD_COUNT
        {
            return Err(CacheError::Config(format!(
                "Thread count must be between {} and {}",
                Self::MIN_THREAD_COUNT,
                Self::MAX_THREAD_COUNT
            )));
        }

        if self.max_image_count < Self::MIN_IMAGE_COUNT
            || self.max_image_count > Self::MAX_IMAGE_COUNT
        {
            return Err(CacheError::Config(format!(
                "Max image count must be between {} and {}",
                Self::MIN_IMAGE_COUNT,
                Self::MAX_IMAGE_COUNT
            )));
        }

        Ok(())
    }
}
