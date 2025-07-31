use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::{instrument, Level};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
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

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level:        LogLevel,
    pub enable_tracy: bool,
    pub log_spans:    bool,
    /// Path to the log file. If None, file logging is disabled.
    pub file_path:    Option<PathBuf>,
}

#[instrument(skip_all)]
pub fn init(config: LogConfig) -> Option<WorkerGuard> {
    // Create console layer
    let console_layer = fmt::layer()
        .with_target(true)
        .with_span_events(if config.log_spans {
            FmtSpan::NEW | FmtSpan::CLOSE
        } else {
            FmtSpan::NONE
        })
        .with_filter(
            EnvFilter::from_default_env().add_directive(
                format!("ferrite={}", level_to_string(config.level))
                    .parse()
                    .expect("Failed to parse log directive"),
            ),
        );

    // Initialize subscriber
    let subscriber = tracing_subscriber::registry().with(console_layer);

    // Add file layer if file_path is provided
    let guard = if let Some(log_file_path) = config.file_path {
        // Extract directory and file name
        let log_dir = log_file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        let file_name_prefix = log_file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or("ferrite");

        let appender = rolling::daily(log_dir, file_name_prefix);
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);

        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_span_events(if config.log_spans {
                FmtSpan::NEW | FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .with_filter(
                EnvFilter::from_default_env().add_directive(
                    format!("ferrite={}", level_to_string(config.level))
                        .parse()
                        .expect("Failed to parse log directive for file"),
                ),
            );

        subscriber.with(file_layer).init();
        Some(guard)
    } else {
        subscriber.init();
        None
    };

    #[cfg(feature = "tracy")]
    if config.enable_tracy {
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::default())
            .init();
    }

    guard
}

fn level_to_string(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    }
}
