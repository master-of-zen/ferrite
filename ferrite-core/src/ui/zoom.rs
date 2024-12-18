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
    fit_mode:   FitMode,
    min_zoom:   f64,
    max_zoom:   f64,
}

impl ZoomHandler {
    pub fn new(default_zoom: f64) -> Self {
        Self {
            zoom_level: default_zoom,
            pan_offset: Vec2::ZERO,
            // Start with FitLonger as the default mode
            fit_mode:   FitMode::FitLonger,
            min_zoom:   0.1,
            max_zoom:   10.0,
        }
    }

    pub fn update_for_new_image(
        &mut self,
        image_size: Vec2,
        window_size: Vec2,
    ) {
        // When a new image is loaded, calculate and apply the appropriate zoom
        // level
        let new_zoom = self.calculate_fit_zoom(image_size, window_size);

        // Always apply the calculated zoom unless we're in Custom mode
        if self.fit_mode != FitMode::Custom {
            self.zoom_level = new_zoom;
            self.pan_offset = Vec2::ZERO;
        }
    }

    pub fn calculate_fit_zoom(
        &self,
        image_size: Vec2,
        window_size: Vec2,
    ) -> f64 {
        // Convert Vec2 (f32) values to f64 for calculation
        let scale_x = (window_size.x / image_size.x) as f64;
        let scale_y = (window_size.y / image_size.y) as f64;

        let zoom = match self.fit_mode {
            FitMode::OneToOne => 1.0,
            FitMode::FitLonger => scale_x.min(scale_y),
            FitMode::FitShorter => scale_x.max(scale_y),
            FitMode::Custom => self.zoom_level,
        };

        // Ensure zoom stays within bounds
        zoom.clamp(self.min_zoom, self.max_zoom)
    }

    pub fn reset_view_position(&mut self) {
        self.pan_offset = Vec2::ZERO;
    }

    pub fn set_zoom(&mut self, new_zoom: f64) {
        self.zoom_level = new_zoom.clamp(self.min_zoom, self.max_zoom);
        // Setting zoom explicitly switches to custom mode
        self.fit_mode = FitMode::Custom;
    }

    pub fn add_offset(&mut self, delta: Vec2) {
        self.pan_offset += delta;
        // When panning, switch to custom mode
        self.fit_mode = FitMode::Custom;
    }

    pub fn reset(&mut self) {
        self.fit_mode = FitMode::OneToOne;
        self.zoom_level = 1.0;
        self.pan_offset = Vec2::ZERO;
    }

    // Getters and setters
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

    pub fn set_fit_mode(&mut self, mode: FitMode) {
        self.fit_mode = mode;
    }
}
