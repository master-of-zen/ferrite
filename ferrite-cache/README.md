# ferrite-cache

Asynchronous image caching system for the Ferrite image viewer.

## Features

- Async image loading and caching using Tokio
- LRU cache eviction strategy
- Image format support via the `image` crate
- Optional performance metrics via `ferrite-metrics` feature
- Comprehensive error handling

## Usage

```rust
use ferrite_cache::{CacheManager, CacheConfig};

let config = CacheConfig {
    max_image_count: 100,
};
let cache = CacheManager::new(config);

// Load and cache an image
let image_data = cache.get_image("path/to/image.jpg").await?;

// Image will be automatically evicted when cache is full
```

## Optional Features

- `ferrite-metrics` - Enables performance monitoring integration with ferrite-logging

## Dependencies

- `tokio` - Async runtime and utilities
- `image` - Image loading and processing
- `tracing` - Logging infrastructure
- `thiserror` - Error handling

## License

MIT License