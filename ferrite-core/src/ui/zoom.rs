use eframe::egui::Vec2;

pub struct ZoomHandler {
    zoom_level: f32,
    pan_offset: Vec2,
}

impl ZoomHandler {
    pub fn new(default_zoom: f32) -> Self {
        Self {
            zoom_level: default_zoom, pan_offset: Vec2::ZERO
        }
    }

    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }

    pub fn zoom_percentage(&self) -> f32 {
        self.zoom_level * 100.0
    }

    pub fn offset(&self) -> Vec2 {
        self.pan_offset
    }

    pub fn add_offset(&mut self, delta: Vec2) {
        self.pan_offset += delta;
    }

    pub fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom_level = new_zoom;
    }

    pub fn reset(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = Vec2::ZERO;
    }
}
