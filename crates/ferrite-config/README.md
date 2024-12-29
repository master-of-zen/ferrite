# Ferrite Config

Configuration management for the Ferrite image viewer.

## Features
- TOML-based configuration file
- Standard XDG configuration paths
- Sensible defaults
- Serialization/deserialization support

## Usage

```rust
use ferrite_config::FeriteConfig;

// Load existing config
let config = FeriteConfig::load().unwrap_or_default();

// Save config
config.save().expect("Failed to save config");

// Access settings
println!("Cache size: {}", config.cache_size);
println!("Default zoom: {}", config.default_zoom);
```