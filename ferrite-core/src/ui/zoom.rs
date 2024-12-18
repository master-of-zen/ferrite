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
    pub fn new(default_zoom: f64, min_zoom: f64, max_zoom: f64) -> Self {
        // Ensure default zoom is within bounds
        let bounded_default = default_zoom.clamp(min_zoom, max_zoom);

        Self {
            zoom_level: bounded_default,
            pan_offset: Vec2::ZERO,
            fit_mode: FitMode::FitLonger,
            min_zoom,
            max_zoom,
        }
    }

    pub fn calculate_fit_zoom(
        &self,
        image_size: Vec2,
        window_size: Vec2,
    ) -> f64 {
        match self.fit_mode {
            FitMode::OneToOne => 1.0,
            FitMode::FitLonger => {
                let scale_x = window_size.x / image_size.x;
                let scale_y = window_size.y / image_size.y;
                scale_x.min(scale_y).into()
            },
            FitMode::FitShorter => {
                let scale_x = window_size.x / image_size.x;
                let scale_y = window_size.y / image_size.y;
                scale_x.max(scale_y).into()
            },
            FitMode::Custom => self.zoom_level,
        }
    }

    pub fn set_fit_mode(&mut self, mode: FitMode) {
        self.fit_mode = mode;
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

    pub fn add_offset(&mut self, delta: Vec2) {
        self.pan_offset += delta;
        // When panning, switch to custom mode as we're no longer in a fit mode
        self.fit_mode = FitMode::Custom;
    }

    pub fn set_zoom(&mut self, new_zoom: f64) {
        self.zoom_level = new_zoom.clamp(self.min_zoom, self.max_zoom);
        self.fit_mode = FitMode::Custom; // Switching to custom mode when
                                         // explicitly setting zoom
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
