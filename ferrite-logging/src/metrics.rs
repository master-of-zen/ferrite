use std::time::{Duration, Instant};
use tracing::{info, info_span};

/// PerformanceMetrics provides utilities for measuring and logging execution
/// times of operations. It creates spans that can be visualized in tracy and
/// logs detailed timing information.
pub struct PerformanceMetrics {
    start_time: Instant,
    operation:  &'static str,
    mark_frame: bool,
}

impl PerformanceMetrics {
    /// Creates a new performance measurement context with optional frame
    /// marking. When frame marking is enabled, it will mark the frame in
    /// tracy's timeline.
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

    /// Finishes the measurement, logs the duration, and optionally marks a
    /// frame.
    pub fn finish(self) -> Duration {
        let duration = self.start_time.elapsed();
        info!(
            operation = self.operation,
            duration_ms = duration.as_millis(),
            "Operation completed"
        );

        // If frame marking is enabled, mark the frame completion
        if self.mark_frame {
            tracy_client::frame_mark();
        }

        duration
    }
}
