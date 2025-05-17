use eframe::egui::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FitMode {
    OneToOne,
    FitLonger,
    FitShorter,
    Custom,
}

pub struct ZoomHandler {
    zoom_level: f64,
    pan_offset: Vec2,
    fit_mode: FitMode,
    default_fit_mode: FitMode,
    manual_zoom_applied: bool,
    min_zoom: f64,
    max_zoom: f64,
}

impl ZoomHandler {
    pub fn new(default_zoom: f64) -> Self {
        Self {
            zoom_level: default_zoom,
            pan_offset: Vec2::ZERO,
            fit_mode: FitMode::FitLonger,
            default_fit_mode: FitMode::FitLonger,
            manual_zoom_applied: false,
            min_zoom: 0.1,
            max_zoom: 10.0,
        }
    }

    pub fn update_for_new_image(
        &mut self,
        image_size: Vec2,
        window_size: Vec2,
    ) {
        self.manual_zoom_applied = false;
        self.fit_mode = self.default_fit_mode;
        let new_zoom = self.calculate_fit_zoom(image_size, window_size);
        self.zoom_level = new_zoom;
        self.pan_offset = Vec2::ZERO;
    }

    pub fn update_for_window_resize(
        &mut self,
        image_size: Vec2,
        window_size: Vec2,
    ) {
        if !self.manual_zoom_applied {
            self.zoom_level = self.calculate_fit_zoom(image_size, window_size);
        }
    }

    pub fn calculate_fit_zoom(
        &self,
        image_size: Vec2,
        window_size: Vec2,
    ) -> f64 {
        let scale_x = (window_size.x / image_size.x) as f64;
        let scale_y = (window_size.y / image_size.y) as f64;

        let zoom = match if self.manual_zoom_applied {
            FitMode::Custom
        } else {
            self.fit_mode
        } {
            FitMode::OneToOne => 1.0,
            FitMode::FitLonger => scale_x.min(scale_y),
            FitMode::FitShorter => scale_x.max(scale_y),
            FitMode::Custom => self.zoom_level,
        };

        zoom.clamp(self.min_zoom, self.max_zoom)
    }

    pub fn set_default_fit_mode(&mut self, mode: FitMode) {
        self.default_fit_mode = mode;
        if !self.manual_zoom_applied {
            self.fit_mode = mode;
        }
    }

    pub fn reset_view_position(&mut self) {
        self.pan_offset = Vec2::ZERO;
    }

    pub fn set_zoom(&mut self, new_zoom: f64) {
        self.zoom_level = new_zoom.clamp(self.min_zoom, self.max_zoom);
        self.manual_zoom_applied = true;
        self.fit_mode = FitMode::Custom;
    }

    pub fn add_offset(&mut self, delta: Vec2) {
        self.pan_offset += delta;
        self.manual_zoom_applied = true;
        self.fit_mode = FitMode::Custom;
    }

    pub fn reset(&mut self) {
        self.manual_zoom_applied = false;
        self.fit_mode = self.default_fit_mode;
        self.zoom_level = 1.0;
        self.pan_offset = Vec2::ZERO;
    }

    pub fn reset_to_default_fit_mode(&mut self) {
        self.manual_zoom_applied = false;
        self.fit_mode = self.default_fit_mode;
        self.pan_offset = Vec2::ZERO;
    }

    pub fn zoom_level(&self) -> f64 {
        self.zoom_level
    }

    pub fn zoom_percentage(&self) -> f64 {
        self.zoom_level * 100.0
    }

    pub fn offset(&self) -> Vec2 {
        self.pan_offset
    }

    pub fn get_fit_mode(&self) -> FitMode {
        self.fit_mode
    }
}
