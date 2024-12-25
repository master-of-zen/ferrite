use clap::Parser;
use ferrite_cache::{CacheConfig, CacheManager};
use std::{path::PathBuf, time::Instant};
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

struct TimingData {
    file_size:   u64,
    memory_size: usize,
    read_time:   std::time::Duration,
    decode_time: std::time::Duration,
    first_load:  std::time::Duration,
    cache_hit:   std::time::Duration,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting read_image tool...");
    let total_start = Instant::now();
    let args = Args::parse();

    let cache_handle = CacheManager::new(CacheConfig {
        max_image_count: args.max_cache,
        thread_count:    4,
    });

    if !args.image.is_file() {
        error!("Not a valid file: {:?}", args.image);
        std::process::exit(1);
    }

    let file_size = std::fs::metadata(&args.image)?.len();

    // First load - cold cache
    let start_time = Instant::now();
    let read_start = Instant::now();
    match cache_handle.get_image(args.image.clone()) {
        Ok(image_data) => {
            let first_load = start_time.elapsed();

            // Test cache hit with multiple iterations
            let mut cache_hits = Vec::new();
            for _ in 0..5 {
                let cache_start = Instant::now();
                let _ = cache_handle.get_image(args.image.clone())?;
                cache_hits.push(cache_start.elapsed());
            }
            let avg_cache_hit = cache_hits.iter().sum::<std::time::Duration>()
                / cache_hits.len() as u32;
            let min_cache_hit = cache_hits.iter().min().unwrap();
            let max_cache_hit = cache_hits.iter().max().unwrap();

            println!("\n📊 Image Details:");

            println!("\n⏱️ Timing Details:");
            println!("   First load: {:.2?}", first_load);
            println!(
                "   Cache hits (avg/min/max): {:.2?}/{:.2?}/{:.2?}",
                avg_cache_hit, min_cache_hit, max_cache_hit
            );
            println!(
                "   Speed improvement: {:.1}x",
                first_load.as_secs_f64() / avg_cache_hit.as_secs_f64()
            );

            if args.detailed {
                println!("\n🔍 Additional Metrics:");
                println!(
                    "   Total execution time: {:.2?}",
                    total_start.elapsed()
                );
            }
        },
        Err(e) => {
            error!("Failed to load image: {}", e);
            std::process::exit(1);
        },
    }

    Ok(())
}
