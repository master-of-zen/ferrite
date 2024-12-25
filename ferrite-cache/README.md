# ferrite-cache

High-performance asynchronous image caching system for the Ferrite image viewer.

## Features

- Asynchronous image loading and decoding using Tokio
- LRU (Least Recently Used) cache eviction strategy
- Efficient memory management with Arc-based sharing
- Image format support via the `image` crate
- Comprehensive error handling and logging
- Optional performance metrics via `ferrite-metrics` feature

## Usage

Add to your Cargo.toml:
```toml
[dependencies]
ferrite-cache = "0.1"
```

### Basic Usage

```rust
use ferrite_cache::{CacheManager, CacheConfig};
use tokio;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize runtime
    let runtime = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap()
    );

    // Create cache manager
    let cache = CacheManager::new(
        CacheConfig { max_image_count: 100 },
        runtime
    );

    // Load and cache an image
    let image = cache.get_image("path/to/image.jpg").await.unwrap();
    println!("Image dimensions: {:?}", image.dimensions());
}
```

### Command Line Tools

The crate includes two binary tools for testing and benchmarking:

#### read_image

Loads and caches a single image, with detailed timing information:

```bash
$ read_image --image test.jpg --detailed
🚀 Starting read_image tool...
📊 Image details:
   Dimensions: 1920x1080
   Raw file size: 2457600 bytes
   Memory size: 8294400 bytes
⏱️ Timing:
   First load: 45.2ms
   Cache hit: 0.8ms
   Speed improvement: 56.5x
```

#### read_directory

Processes and caches all images in a directory with parallel loading:

```bash
$ read_directory --directory ./photos --max-images 1000 --verbose
🚀 Starting directory scanner...
📊 Summary:
  - Images processed: 50
  - Successfully cached: 50
  - Total memory usage: 154.2 MB
  - Average cache hit time: 0.9ms
```

## Performance Tips

1. **Cache Size**: Set `max_image_count` based on available memory and typical image sizes
2. **Worker Threads**: Adjust runtime worker threads based on CPU cores
3. **Preloading**: Use read_directory for batch preloading of frequently accessed images

## Error Handling

The crate provides detailed error types:
- `CacheError::ImageLoad` - File reading or format errors
- `CacheError::CapacityExceeded` - Cache size limits
- `CacheError::FileSystem` - OS-level file operations
- `CacheError::Config` - Configuration issues

## Contributing

Contributions welcome! Please check out our [Contributing Guide](CONTRIBUTING.md).

## License

MIT License
