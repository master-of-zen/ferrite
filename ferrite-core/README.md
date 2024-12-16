# ferrite-core

Core functionality for the Ferrite image viewer. This crate provides the essential building blocks for image viewing, manipulation, and navigation.

## Features

* Fast image loading with LRU caching
* Smooth zooming and panning
* Directory-based image navigation
* Configurable UI elements

## Architecture

The crate is organized into several modules:

- `image/` - Image loading, caching, and management
- `ui/` - User interface components and rendering
- `navigation/` - Directory traversal and image navigation

## Usage

```rust
use ferrite_core::FeriteApp;
use ferrite_config::FeriteConfig;

fn main() {
    let config = FeriteConfig::default();
    let app = FeriteApp::new(
        &eframe::CreationContext::default(),
        Some("path/to/image.jpg".into()),
        config,
    );
}
```

## Dependencies

- `eframe`, `egui` - GUI framework
- `image` - Image processing
- `lru` - Cache management
- `tracing` - Logging and diagnostics
- `ferrite-config` - Configuration management

## License

Same as Ferrite main project