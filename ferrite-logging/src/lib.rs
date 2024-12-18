use std::str::FromStr;
use tracing::{info, instrument, Level};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
    Registry,
};

// Export our new metrics module
pub mod metrics;
pub use metrics::PerformanceMetrics;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

#[derive(Debug)]
pub struct LogConfig {
    pub level:        LogLevel,
    pub enable_tracy: bool,
    // Adding new configuration options for performance logging
    pub log_spans:    bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level:        LogLevel::Info,
            enable_tracy: false,
            log_spans:    true,
        }
    }
}

#[instrument]
pub fn init(config: LogConfig) {
    let level = Level::from(config.level);
    let filter = tracing_subscriber::filter::LevelFilter::from_level(level);

    let fmt_layer = fmt::layer()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .with_span_events(if config.log_spans {
            FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT
        } else {
            FmtSpan::NONE
        })
        .with_filter(filter.clone());

    let registry = Registry::default().with(fmt_layer);

    if config.enable_tracy {
        // Create the tracy layer with the correct configuration
        let tracy_layer = tracing_tracy::TracyLayer::new().with_filter(filter);

        registry
            .with(tracy_layer)
            .try_init()
            .expect("Failed to initialize logging with tracy");

        // After initialization, we can use tracy-client directly for frame
        // marking
        tracy_client::frame_mark();
    } else {
        registry
            .try_init()
            .expect("Failed to initialize logging");
    }
}

#[instrument]
pub fn get_log_level_from_env() -> LogLevel {
    std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(LogLevel::Info)
}
