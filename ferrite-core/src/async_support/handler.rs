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
    pub fn new(runtime: Runtime) -> Self {
        let runtime = Arc::new(runtime);
        let config = CacheConfig::default();
        let cache_manager = Arc::new(CacheManager::new(config));

        Self {
            runtime,
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
                                    let bytes = image_data.data();
                                    match image::load_from_memory(&bytes) {
                                        Ok(img) => {
                                            let _ = response_tx
                                                .send(ImageResponse::Loaded(
                                                    path, img,
                                                ))
                                                .await;
                                        },
                                        Err(err) => {
                                            warn!(
                                                "Failed to decode image: {:?}",
                                                err
                                            );
                                            let _ = response_tx
                                                .send(ImageResponse::Error(
                                                    path,
                                                ))
                                                .await;
                                        },
                                    }
                                },
                                Err(err) => {
                                    warn!(
                                        "Failed to load image from cache: {:?}",
                                        err
                                    );
                                    let _ = response_tx
                                        .send(ImageResponse::Error(path))
                                        .await;
                                },
                            }
                        },
                        ImageRequest::Clear => {
                            info!("Clearing image cache");
                        },
                    }
                }
            });
        });
    }
}
