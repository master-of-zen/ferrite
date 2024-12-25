use clap::Parser;
use ferrite_cache::{CacheConfig, CacheHandle, CacheManager};
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_name = "DIR")]
    directory: PathBuf,

    #[arg(short, long, default_value_t = 100)]
    max_images: usize,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let cache_handle = CacheManager::new(CacheConfig {
        max_image_count: args.max_images,
        thread_count:    4,
    });

    if !args.directory.is_dir() {
        error!("Not a valid directory: {:?}", args.directory);
        std::process::exit(1);
    }

    // Collect image paths
    let mut image_paths = Vec::new();
    for entry in std::fs::read_dir(&args.directory)? {
        let path = entry?.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ["jpg", "jpeg", "png", "gif"]
                    .contains(&ext.to_lowercase().as_str())
                {
                    image_paths.push(path);
                }
            }
        }
    }

    println!("Found {} images", image_paths.len());

    // Cache all images
    println!("Caching images...");
    let mut cached = 0;
    for path in &image_paths {
        if cache_handle.cache_image(path.clone()).is_ok() {
            cached += 1;
            if args.verbose {
                println!("✓ Cached: {:?}", path);
            }
        }
    }

    // Test retrievals
    println!("Testing cache...");
    let mut retrieved = 0;
    for path in &image_paths {
        if cache_handle.get_image(path.clone()).is_ok() {
            retrieved += 1;
            if args.verbose {
                println!("✓ Retrieved: {:?}", path);
            }
        }
    }

    println!("\nSummary:");
    println!("Images found: {}", image_paths.len());
    println!("Successfully cached: {}", cached);
    println!("Successfully retrieved: {}", retrieved);

    Ok(())
}
