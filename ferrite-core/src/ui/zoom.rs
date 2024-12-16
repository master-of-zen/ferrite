use eframe::egui::{self, Context, Key, Pos2, Rect, Vec2};

pub struct ZoomHandler {
    zoom_level: f32,
    offset: Vec2,
    image_rect: Option<Rect>,
}

impl ZoomHandler {
    pub fn new(default_zoom: f32) -> Self {
        Self {
            zoom_level: default_zoom,
            offset: Vec2::ZERO,
            image_rect: None,
        }
    }

    pub fn handle_keyboard_input(&mut self, ctx: &Context) {
        let cursor_pos = ctx.pointer_hover_pos();

        // Handle keyboard zoom
        if ctx.input(|i| i.key_pressed(Key::Equals) || i.key_pressed(Key::Plus)) {
            self.keyboard_zoom(1.1);
        }
        if ctx.input(|i| i.key_pressed(Key::Minus)) {
            self.keyboard_zoom(1.0 / 1.1);
        }
        if ctx.input(|i| i.key_pressed(Key::Num0)) {
            self.zoom_level = 1.0;
            self.offset = Vec2::ZERO;
        }

        // Handle scroll wheel zoom
        let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 1.0 / 1.1 };
            if let Some(cursor) = cursor_pos {
                if let Some(rect) = self.image_rect {
                    if rect.contains(cursor) {
                        self.cursor_zoom(cursor, zoom_factor);
                        return;
                    }
                }
            }
            self.center_zoom(zoom_factor);
        }
    }

    fn keyboard_zoom(&mut self, factor: f32) {
        if let Some(rect) = self.image_rect {
            let center = rect.center();
            self.cursor_zoom(center.into(), factor);
        }
    }

    fn cursor_zoom(&mut self, cursor: Pos2, factor: f32) {
        let old_zoom = self.zoom_level;
        self.zoom_level = (self.zoom_level * factor).clamp(0.1, 10.0);

        let cursor_vec = Vec2::new(cursor.x, cursor.y);
        let zoom_center = cursor_vec - self.offset;
        self.offset = cursor_vec - (zoom_center * (self.zoom_level / old_zoom));
    }

    fn center_zoom(&mut self, factor: f32) {
        if let Some(rect) = self.image_rect {
            let center = rect.center();
            self.cursor_zoom(center.into(), factor);
        } else {
            self.zoom_level = (self.zoom_level * factor).clamp(0.1, 10.0);
        }
    }

    pub fn set_image_rect(&mut self, rect: Rect) {
        self.image_rect = Some(rect);
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
