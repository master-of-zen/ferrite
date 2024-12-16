use eframe::egui::{self, Context, Key, Rect, Ui, Vec2};

pub struct ZoomHandler {
    zoom_level: f32,
    // Renamed from drag_offset to clarify its purpose
    pan_offset: Vec2,
}

impl ZoomHandler {
    pub fn new(default_zoom: f32) -> Self {
        Self {
            zoom_level: default_zoom,
            pan_offset: Vec2::ZERO,
        }
    }

    pub fn handle_keyboard_input(&mut self, ctx: &Context, ui: &Ui) {
        // Handle keyboard-based zoom
        if ctx.input(|i| i.key_pressed(Key::Equals) || i.key_pressed(Key::Plus)) {
            self.handle_zoom(ui, 1.0); // Zoom in
        }
        if ctx.input(|i| i.key_pressed(Key::Minus)) {
            self.handle_zoom(ui, -1.0); // Zoom out
        }
        if ctx.input(|i| i.key_pressed(Key::Num0)) {
            // Reset zoom and position
            self.zoom_level = 1.0;
            self.pan_offset = Vec2::ZERO;
            ctx.request_repaint();
        }

        // Handle scroll wheel zoom
        let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            self.handle_zoom(ui, scroll_delta);
        }
    }

    fn handle_zoom(&mut self, ui: &Ui, scroll_delta: f32) {
        // Get current mouse position relative to the screen
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            let panel_rect = ui.available_rect_before_wrap();

            // Calculate the center of the image including current pan
            let image_center = panel_rect.center() + self.pan_offset;

            // Calculate the mouse position relative to the image center before zoom
            let mouse_from_center = mouse_pos - image_center;

            // Store old zoom to calculate the zoom change ratio
            let old_zoom = self.zoom_level;

            // Calculate new zoom level with smoother steps
            let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            self.zoom_level = (self.zoom_level * zoom_factor).clamp(0.1, 10.0);

            // Calculate the scaling ratio between old and new zoom
            let zoom_ratio = self.zoom_level / old_zoom;

            // Calculate how much the position under the mouse should move
            let scaled_mouse_offset = mouse_from_center * (zoom_ratio - 1.0);

            // Update the pan offset to maintain the mouse position relative to the image
            self.pan_offset -= scaled_mouse_offset;

            // Request a repaint to ensure smooth visual updates
            ui.ctx().request_repaint();
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
}
