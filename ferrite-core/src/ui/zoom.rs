use eframe::egui::{self, Context, Key, Vec2};

pub struct ZoomHandler {
    zoom_level: f32,
    offset: Vec2,
}

impl ZoomHandler {
    pub fn new(default_zoom: f32) -> Self {
        Self {
            zoom_level: default_zoom,
            offset: Vec2::ZERO,
        }
    }

    pub fn handle_keyboard_input(&mut self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(Key::Equals) || i.key_pressed(Key::Plus)) {
            self.zoom_level = (self.zoom_level * 1.1).min(10.0);
        }
        if ctx.input(|i| i.key_pressed(Key::Minus)) {
            self.zoom_level = (self.zoom_level / 1.1).max(0.1);
        }
        if ctx.input(|i| i.key_pressed(Key::Num0)) {
            self.zoom_level = 1.0;
            self.offset = Vec2::ZERO;
        }
    }

    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }

    pub fn zoom_percentage(&self) -> f32 {
        self.zoom_level * 100.0
    }

    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    pub fn add_offset(&mut self, delta: Vec2) {
        self.offset += delta;
    }
}
