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
            fit_mode:   FitMode::FitLonger, // Default to FitLonger mode
            min_zoom:   0.1,
            max_zoom:   10.0,
        }
    }

    /// Recalculates zoom level based on current fit mode and image dimensions
    pub fn update_for_new_image(
        &mut self,
        image_size: Vec2,
        window_size: Vec2,
    ) {
        // Only recalculate if not in Custom mode
        if self.fit_mode != FitMode::Custom {
            self.zoom_level = self
                .calculate_fit_zoom(image_size, window_size)
                .into();
            self.pan_offset = Vec2::ZERO; // Reset pan offset for new image
        }
    }

    /// Not correct, need to be redone
    pub fn calculate_fit_zoom(
        &self,
        image_size: Vec2,
        window_size: Vec2,
    ) -> f64 {
        match self.fit_mode {
            FitMode::OneToOne => 1.0,
            FitMode::FitLonger => {
                let scale_x = (window_size.x / image_size.x) as f64;
                let scale_y = (window_size.y / image_size.y) as f64;
                scale_x
                    .min(scale_y)
                    .clamp(self.min_zoom, self.max_zoom)
            },
            FitMode::FitShorter => {
                let scale_x = (window_size.x / image_size.x) as f64;
                let scale_y = (window_size.y / image_size.y) as f64;
                scale_x
                    .max(scale_y)
                    .clamp(self.min_zoom, self.max_zoom)
            },
            FitMode::Custom => self.zoom_level,
        }
    }

    // Existing methods remain the same
    pub fn zoom_level(&self) -> f64 {
        self.zoom_level
    }

    pub fn zoom_percentage(&self) -> f64 {
        self.zoom_level * 100.0
    }

    pub fn offset(&self) -> Vec2 {
        self.pan_offset
    }

    pub fn add_offset(&mut self, delta: Vec2) {
        self.pan_offset += delta;
        // When panning, switch to custom mode
        self.fit_mode = FitMode::Custom;
    }

    pub fn set_zoom(&mut self, new_zoom: f64) {
        self.zoom_level = new_zoom.clamp(self.min_zoom, self.max_zoom);
        // Setting zoom explicitly switches to custom mode
        self.fit_mode = FitMode::Custom;
    }

    pub fn set_fit_mode(&mut self, mode: FitMode) {
        self.fit_mode = mode;
    }

    pub fn reset(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = Vec2::ZERO;
        self.fit_mode = FitMode::OneToOne;
    }

    pub fn get_fit_mode(&self) -> FitMode {
        self.fit_mode
    }
}
