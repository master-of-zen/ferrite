use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{instrument, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    Layer, // Registry is implicitly used by .with() chain
};

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
    pub level: LogLevel,
    pub enable_tracy: bool,
    pub log_spans: bool,
    pub file_path: Option<PathBuf>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_tracy: false,
            log_spans: true,
            file_path: None,
        }
    }
}

#[instrument(skip(config), fields(level = ?config.level, tracy = config.enable_tracy, file = ?config.file_path, spans = config.log_spans))]
pub fn init(config: LogConfig) -> Option<WorkerGuard> {
    let base_level = Level::from(config.level);

    // Helper closure to create EnvFilter instances
    // This ensures RUST_LOG is read and parsed for each layer,
    // which is fine for startup.
    let create_env_filter = || {
        EnvFilter::builder()
            .with_default_directive(base_level.into())
            .from_env_lossy()
    };

    let span_events =
        if config.log_spans { FmtSpan::CLOSE } else { FmtSpan::NONE };

    // Console Layer
    let console_fmt_layer = fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_timer(fmt::time::ChronoUtc::rfc_3339())
        .with_span_events(span_events.clone())
        .with_filter(create_env_filter()); // Use a new filter instance

    // File Layer (optional)
    let mut file_guard: Option<WorkerGuard> = None;
    let file_fmt_layer_maybe = if let Some(log_path) = &config.file_path {
        let parent_dir = log_path
            .parent()
            .unwrap_or_else(|| Path::new("."));

        if !parent_dir.as_os_str().is_empty() && !parent_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(parent_dir) {
                eprintln!(
                    "Error: Failed to create log directory {}: {}. File logging will be disabled.",
                    parent_dir.display(),
                    e
                );
                None
            } else {
                let file_name = log_path
                    .file_name()
                    .unwrap_or_else(|| OsStr::new("ferrite.log"));
                let file_appender = rolling::never(parent_dir, file_name);
                let (non_blocking_writer, guard) =
                    tracing_appender::non_blocking(file_appender);
                file_guard = Some(guard);

                Some(
                    fmt::layer()
                        .with_ansi(false)
                        .with_writer(non_blocking_writer)
                        .with_target(true)
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_ids(true)
                        .with_timer(fmt::time::ChronoUtc::rfc_3339())
                        .with_span_events(span_events.clone())
                        .with_filter(create_env_filter()), // Use a new filter instance
                )
            }
        } else {
            let file_name = log_path
                .file_name()
                .unwrap_or_else(|| OsStr::new("ferrite.log"));
            let effective_parent_dir = if parent_dir.as_os_str().is_empty() {
                Path::new(".")
            } else {
                parent_dir
            };
            let file_appender = rolling::never(effective_parent_dir, file_name);
            let (non_blocking_writer, guard) =
                tracing_appender::non_blocking(file_appender);
            file_guard = Some(guard);

            Some(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(non_blocking_writer)
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_timer(fmt::time::ChronoUtc::rfc_3339())
                    .with_span_events(span_events)
                    .with_filter(create_env_filter()), // Use a new filter instance
            )
        }
    } else {
        None
    };

    // Tracy Layer (optional)
    let tracy_layer_maybe = if config.enable_tracy {
        Some(
            tracing_tracy::TracyLayer::default()
                .with_filter(create_env_filter()),
        ) // Use a new filter instance
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(console_fmt_layer)
        .with(file_fmt_layer_maybe)
        .with(tracy_layer_maybe)
        .try_init()
        .expect("Failed to initialize logging subscriber");

    if config.enable_tracy {
        tracy_client::frame_mark();
    }

    file_guard
}
