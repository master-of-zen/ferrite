use image::DynamicImage;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub enum ImageRequest {
    Load(PathBuf),
    Clear,
}

pub enum ImageResponse {
    Loaded(PathBuf, DynamicImage),
    Error(PathBuf),
}

pub type ImageSender = mpsc::Sender<ImageRequest>;
pub type ImageReceiver = mpsc::Receiver<ImageRequest>;
pub type ResponseSender = mpsc::Sender<ImageResponse>;
pub type ResponseReceiver = mpsc::Receiver<ImageResponse>;

pub struct AsyncChannels {
    pub request_tx:  ImageSender,
    pub response_rx: ResponseReceiver,
}

impl AsyncChannels {
    pub fn new(capacity: usize) -> (Self, ImageReceiver, ResponseSender) {
        let (request_tx, request_rx) = mpsc::channel(capacity);
        let (response_tx, response_rx) = mpsc::channel(capacity);

        (
            Self {
                request_tx,
                response_rx,
            },
            request_rx,
            response_tx,
        )
    }
}
