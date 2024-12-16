use eframe::egui;
use image::DynamicImage;

pub struct ImageData {
    pub(crate) texture: Option<egui::TextureHandle>,
    pub(crate) original: DynamicImage,
}

impl ImageData {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            texture: None,
            original: image,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.original.width(), self.original.height())
    }
}
