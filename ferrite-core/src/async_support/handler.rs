use super::message::{
    ImageReceiver,
    ImageRequest,
    ImageResponse,
    ResponseSender,
};
use ferrite_cache::{CacheConfig, CacheManager};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tracing::{info, warn};

pub struct AsyncHandler {
    runtime:       Arc<Runtime>,
    cache_manager: Arc<CacheManager>,
}

impl AsyncHandler {
    pub fn new(
        runtime: Arc<Runtime>,
        cache_manager: Arc<CacheManager>,
    ) -> Self {
        Self {
            runtime: runtime.clone(),
            cache_manager,
        }
    }

    pub fn spawn_handler(
        &self,
        mut request_rx: ImageReceiver,
        response_tx: ResponseSender,
    ) {
        let runtime = self.runtime.clone();
        let cache_manager = self.cache_manager.clone();

        std::thread::spawn(move || {
            runtime.block_on(async move {
                while let Some(request) = request_rx.recv().await {
                    match request {
                        ImageRequest::Load(path) => {
                            match cache_manager.get_image(path.clone()).await {
                                Ok(image_data) => {
                                    match image::load_from_memory(
                                        &image_data.data(),
                                    ) {
                                        Ok(img) => {
                                            let _ = response_tx
                                                .send(ImageResponse::Loaded(
                                                    path, img,
                                                ))
                                                .await;
                                        },
                                        Err(e) => {
                                            tracing::warn!(
                                                "Failed to decode image: {:?}",
                                                e
                                            );
                                            let _ = response_tx
                                                .send(ImageResponse::Error(
                                                    path,
                                                ))
                                                .await;
                                        },
                                    }
                                },
                                Err(e) => {
                                    tracing::warn!("Cache error: {:?}", e);
                                    let _ = response_tx
                                        .send(ImageResponse::Error(path))
                                        .await;
                                },
                            }
                        },
                        ImageRequest::Clear => {
                            tracing::info!("Clearing image cache");
                        },
                    }
                }
            });
        });
    }
}
