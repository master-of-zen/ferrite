use crate::{
    defaults::zoom::*,
    error::{ConfigError, Result},
};
use serde::{Deserialize, Serialize};

/// Validated, ordered set of zoom steps
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ZoomSteps(Vec<f64>);

impl ZoomSteps {
    pub fn new(mut steps: Vec<f64>) -> Result<Self> {
        if steps.is_empty() {
            return Err(ConfigError::ValidationError(
                "Zoom steps cannot be empty".into(),
            ));
        }
        steps.sort_by(|a, b| {
            a.partial_cmp(b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        steps.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
        Ok(Self(steps))
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.0
    }
}

impl Default for ZoomSteps {
    fn default() -> Self {
        Self::new(DEFAULT_ZOOM_STEPS.to_vec())
            .expect("Default zoom steps must be valid")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FitMode {
    /// Display image at actual size (100% zoom)
    OneToOne,
    /// Fit image to window, scaling by the longer dimension
    FitLonger,
    /// Fit image to window, scaling by the shorter dimension
    FitShorter,
    /// Use custom zoom level
    Custom,
}

impl Default for FitMode {
    fn default() -> Self {
        FitMode::FitLonger
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomConfig {
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub default_zoom: f64,
    pub zoom_step: f64,
    pub use_predefined_steps: bool,
    pub zoom_steps: ZoomSteps,
    pub focal_point_enabled: bool,
    pub transition_enabled: bool,
    pub transition_duration: f64,
    pub fit_to_window: bool,
    pub maintain_aspect_ratio: bool,
    pub default_fit_mode: FitMode,
}

impl Default for ZoomConfig {
    fn default() -> Self {
        Self {
            min_zoom: MIN_ZOOM,
            max_zoom: MAX_ZOOM,
            default_zoom: DEFAULT_ZOOM,
            zoom_step: ZOOM_STEP,
            use_predefined_steps: USE_PREDEFINED_STEPS,
            zoom_steps: ZoomSteps::default(),
            focal_point_enabled: FOCAL_POINT_ENABLED,
            transition_enabled: TRANSITION_ENABLED,
            transition_duration: TRANSITION_DURATION,
            fit_to_window: FIT_TO_WINDOW,
            maintain_aspect_ratio: MAINTAIN_ASPECT_RATIO,
            default_fit_mode: FitMode::default(),
        }
    }
}

impl ZoomConfig {
    pub fn validate(&self) -> Result<()> {
        if self.min_zoom <= 0.0 {
            return Err(ConfigError::ValidationError(
                "min_zoom must be positive".into(),
            ));
        }

        if self.max_zoom <= self.min_zoom {
            return Err(ConfigError::ValidationError(format!(
                "max_zoom ({}) must be greater than min_zoom ({})",
                self.max_zoom, self.min_zoom
            )));
        }

        if self.default_zoom < self.min_zoom
            || self.default_zoom > self.max_zoom
        {
            return Err(ConfigError::ValidationError(format!(
                "default_zoom must be between {} and {}",
                self.min_zoom, self.max_zoom
            )));
        }

        if self.zoom_step <= 0.0 {
            return Err(ConfigError::ValidationError(
                "zoom_step must be positive".into(),
            ));
        }

        if self.transition_duration < 0.0 {
            return Err(ConfigError::ValidationError(
                "transition_duration cannot be negative".into(),
            ));
        }

        if self.use_predefined_steps {
            for &step in self.zoom_steps.as_slice() {
                if step < self.min_zoom || step > self.max_zoom {
                    return Err(ConfigError::ValidationError(format!(
                        "zoom step {} is outside allowed range [{}, {}]",
                        step, self.min_zoom, self.max_zoom
                    )));
                }
            }
        }

        Ok(())
    }
}
