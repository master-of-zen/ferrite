use std::time::{Duration, Instant};
use tracing::{info, info_span};

pub struct PerformanceMetrics {
    start_time: Instant,
    operation:  &'static str,
    mark_frame: bool,
}

impl PerformanceMetrics {
    pub fn new(operation: &'static str, mark_frame: bool) -> Self {
        let span = info_span!("performance_measurement",
            operation = operation,
            start_time = ?Instant::now()
        );

        span.in_scope(|| {
            info!("Starting operation: {}", operation);
        });

        Self {
            start_time: Instant::now(),
            operation,
            mark_frame,
        }
    }

    pub fn finish(self) -> Duration {
        let duration = self.start_time.elapsed();
        info!(
            operation = self.operation,
            duration_us = duration.as_micros(),
            "Operation completed"
        );

        // If frame marking is enabled, mark the frame completion
        if self.mark_frame {
            tracy_client::frame_mark();
        }

        duration
    }
}
