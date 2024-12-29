# Ferrite Logging

Logging infrastructure for the Ferrite image viewer.

## Features
- Log level configuration through environment variables and CLI
- Tracy profiler integration for performance monitoring
- Simple initialization API
- Type-safe log level handling

## Usage

```rust
use ferrite_logging::{LogConfig, LogLevel, init};

// Basic setup with default config
let config = LogConfig::default();
init(config);

// Custom setup with Tracy enabled
let config = LogConfig {
    level: LogLevel::Debug,
    enable_tracy: true,
};
init(config);

// Get log level from RUST_LOG environment variable
let level = ferrite_logging::get_log_level_from_env();
```