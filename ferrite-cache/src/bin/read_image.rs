use clap::Parser;
use ferrite_cache::{CacheConfig, CacheManager};
use std::path::PathBuf;
use tracing::error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    image: PathBuf,

    #[arg(short, long)]
    detailed: bool,

    #[arg(short, long, default_value_t = 100)]
    max_cache: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting read_image tool...");
    let args = Args::parse();

    let cache_handle = CacheManager::new(CacheConfig {
        max_image_count: args.max_cache,
        thread_count:    4,
    });

    if !args.image.is_file() {
        error!("Not a valid file: {:?}", args.image);
        std::process::exit(1);
    }

    // First load - cold cache
    let start_time = std::time::Instant::now();
    match cache_handle.get_image(args.image.clone()) {
        Ok(image_data) => {
            let first_load = start_time.elapsed();
            let dims = image_data.dimensions();

            // Test cache hit
            let cache_start = std::time::Instant::now();
            let _ = cache_handle.get_image(args.image.clone())?;
            let cache_hit = cache_start.elapsed();

            println!("\n📊 Results:");
            println!("   Dimensions: {}x{}", dims.0, dims.1);
            println!("   First load: {:?}", first_load);
            println!("   Cache hit: {:?}", cache_hit);
            println!(
                "   Speed improvement: {:.1}x",
                first_load.as_secs_f64() / cache_hit.as_secs_f64()
            );
        },
        Err(e) => {
            error!("Failed to load image: {}", e);
            std::process::exit(1);
        },
    }

    Ok(())
}
