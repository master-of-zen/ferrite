// src/lib.rs
use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::prelude::*;

/// Log levels supported by the application
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

/// Configuration for logging setup
#[derive(Debug)]
pub struct LogConfig {
    pub level: LogLevel,
    pub enable_tracy: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_tracy: false,
        }
    }
}

/// Initialize logging with the provided configuration
pub fn init(config: LogConfig) {
    let level = Level::from(config.level);
    let filter = tracing_subscriber::filter::LevelFilter::from_level(level);

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter.clone()));

    if config.enable_tracy {
        let subscriber = subscriber.with(tracing_tracy::TracyLayer::new().with_filter(filter));
        subscriber.init();
    } else {
        subscriber.init();
    }
}

/// Parse log level from environment or default to Info
pub fn get_log_level_from_env() -> LogLevel {
    std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(LogLevel::Info)
}
