use clap::Parser;
use ferrite_cache::{CacheConfig, CacheHandle, CacheManager};
use std::{collections::HashMap, path::PathBuf, time::Instant};
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

#[derive(Default)]
struct ImageStats {
    file_size:      u64,
    memory_size:    usize,
    dimensions:     (u32, u32),
    cache_time:     f64,
    retrieval_time: f64,
}

struct DirectoryStats {
    total_start:       Instant,
    scan_time:         std::time::Duration,
    image_stats:       HashMap<PathBuf, ImageStats>,
    cache_times:       Vec<f64>,
    retrieval_times:   Vec<f64>,
    total_file_size:   u64,
    total_memory_size: usize,
}

impl DirectoryStats {
    fn new() -> Self {
        Self {
            total_start:       Instant::now(),
            scan_time:         std::time::Duration::default(),
            image_stats:       HashMap::new(),
            cache_times:       Vec::new(),
            retrieval_times:   Vec::new(),
            total_file_size:   0,
            total_memory_size: 0,
        }
    }

    fn print_summary(&self, verbose: bool) {
        let total_time = self.total_start.elapsed();
        let successful_count = self.image_stats.len();

        println!("\n📊 Operation Summary:");
        println!("   Directory scan time: {:.2?}", self.scan_time);
        println!("   Images processed: {}", successful_count);
        println!(
            "   Total file size: {:.2} MB",
            self.total_file_size as f64 / 1_048_576.0
        );
        println!(
            "   Total memory usage: {:.2} MB",
            self.total_memory_size as f64 / 1_048_576.0
        );
        println!(
            "   Memory overhead: {:.1}%",
            (self.total_memory_size as f64 / self.total_file_size as f64 - 1.0)
                * 100.0
        );

        println!("\n⏱️ Timing Statistics:");
        if !self.cache_times.is_empty() {
            let avg_cache = self.cache_times.iter().sum::<f64>()
                / self.cache_times.len() as f64;
            let min_cache = self
                .cache_times
                .iter()
                .fold(f64::INFINITY, |a, &b| a.min(b));
            let max_cache = self
                .cache_times
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            println!("Cache Operations:");
            println!("   Average: {:.2}ms", avg_cache);
            println!("   Minimum: {:.2}ms", min_cache);
            println!("   Maximum: {:.2}ms", max_cache);
        }

        if !self.retrieval_times.is_empty() {
            let avg_retrieval = self.retrieval_times.iter().sum::<f64>()
                / self.retrieval_times.len() as f64;
            let min_retrieval = self
                .retrieval_times
                .iter()
                .fold(f64::INFINITY, |a, &b| a.min(b));
            let max_retrieval = self
                .retrieval_times
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            println!("Retrieval Operations:");
            println!("   Average: {:.2}ms", avg_retrieval);
            println!("   Minimum: {:.2}ms", min_retrieval);
            println!("   Maximum: {:.2}ms", max_retrieval);
            println!(
                "   Speed improvement: {:.1}x",
                self.cache_times.iter().sum::<f64>()
                    / self.retrieval_times.iter().sum::<f64>()
            );
        }

        if verbose {
            println!("\n🔍 Image Details:");
            for (path, stats) in &self.image_stats {
                println!("\nFile: {:?}", path);
                println!(
                    "   Dimensions: {}x{}",
                    stats.dimensions.0, stats.dimensions.1
                );
                println!("   File size: {} KB", stats.file_size / 1024);
                println!("   Memory size: {} KB", stats.memory_size / 1024);
                println!("   Cache time: {:.2}ms", stats.cache_time);
                println!("   Retrieval time: {:.2}ms", stats.retrieval_time);
                println!(
                    "   Speed improvement: {:.1}x",
                    stats.cache_time / stats.retrieval_time
                );
            }
        }

        println!("\n📈 Performance Summary:");
        println!("   Total execution time: {:.2?}", total_time);
        println!(
            "   Average processing time per image: {:.2?}",
            total_time.div_f64(successful_count as f64)
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut stats = DirectoryStats::new();

    println!("🚀 Starting directory scanner...");

    let cache_handle = CacheManager::new(CacheConfig {
        max_image_count: args.max_images,
        thread_count:    4,
    });

    if !args.directory.is_dir() {
        error!("Not a valid directory: {:?}", args.directory);
        std::process::exit(1);
    }

    // Collect image paths
    let start_scan = Instant::now();
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
    stats.scan_time = start_scan.elapsed();
    println!("Found {} images in {:.2?}", image_paths.len(), stats.scan_time);

    // Process all images
    println!("\nProcessing images...");
    for path in &image_paths {
        let file_size = std::fs::metadata(path)?.len();
        stats.total_file_size += file_size;

        let mut image_stats = ImageStats {
            file_size,
            ..Default::default()
        };

        // Cache image
        let start = Instant::now();
        if let Ok(image_data) = cache_handle.cache_image(path.clone()) {
            let duration = start.elapsed();
            let cache_time = duration.as_secs_f64() * 1000.0;
            stats.cache_times.push(cache_time);

            image_stats.cache_time = cache_time;
            stats.total_memory_size += image_stats.memory_size;

            if args.verbose {
                println!("✓ Cached: {:?} ({:.2?})", path, duration);
            }c

            // Test retrieval
            let start = Instant::now();
            if let Ok(_) = cache_handle.get_image(path.clone()) {
                let duration = start.elapsed();
                let retrieval_time = duration.as_secs_f64() * 1000.0;
                stats.retrieval_times.push(retrieval_time);
                image_stats.retrieval_time = retrieval_time;

                if args.verbose {
                    println!("✓ Retrieved: {:?} ({:.2?})", path, duration);
                }
            }

            stats
                .image_stats
                .insert(path.clone(), image_stats);
        }
    }

    stats.print_summary(args.verbose);

    Ok(())
}
