use clap::Parser;
use ferrite_cache::{CacheConfig, CacheManager};
use once_cell::sync::OnceCell;
use std::{path::PathBuf, sync::Arc, time::Instant};
use tokio;
use tracing::{error, info};

static CACHE_MANAGER: OnceCell<Arc<CacheManager>> = OnceCell::new();

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
    println!("📋 Args parsed: {:?}", args);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(12)
        .enable_all()
        .build()?;

    println!("⚙️ Runtime created with 12 worker threads");
    let runtime_arc = Arc::new(runtime);

    let cache_manager = CACHE_MANAGER.get_or_init(|| {
        println!(
            "🏗️ Initializing cache manager with max {} images",
            args.max_cache
        );
        Arc::new(CacheManager::new(
            CacheConfig {
                max_image_count: args.max_cache
            },
            runtime_arc.clone(),
        ))
    });

    println!("▶️ Starting async execution");
    runtime_arc.block_on(async {
        run_image_load(args, cache_manager.clone()).await
    })?;

    println!("✨ Execution completed successfully");
    Ok(())
}

async fn run_image_load(
    args: Args,
    cache_manager: Arc<CacheManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.image.is_file() {
        println!("❌ Not a valid file: {:?}", args.image);
        error!("Not a valid file: {:?}", args.image);
        std::process::exit(1);
    }

    println!("📖 Loading image: {:?}", args.image);
    let total_start = Instant::now();
    match cache_manager.get_image(args.image.clone()).await {
        Ok(image_data) => {
            let first_load_duration = total_start.elapsed();
            let dims = image_data.dimensions();
            let data_size = image_data.data().len();
            let file_size = args.image.metadata()?.len();

            println!("🔄 Testing cache by loading again...");
            let cache_start = Instant::now();
            let _cached_image = cache_manager
                .get_image(args.image.clone())
                .await?;
            let cache_duration = cache_start.elapsed();

            println!("✅ Image loaded successfully: {:?}", args.image);
            println!("⏱️ Timing information:");
            println!("   First load (total): {:?}", first_load_duration);
            println!("   Cache hit time: {:?}", cache_duration);
            println!(
                "   Speed improvement: {:.1}x",
                first_load_duration.as_secs_f64()
                    / cache_duration.as_secs_f64()
            );

            println!("📊 Image details:");
            println!("   Dimensions: {}x{}", dims.0, dims.1);
            println!("   Raw file size: {} bytes", file_size);
            println!("   Memory size: {} bytes", data_size);
            println!(
                "   Memory/File ratio: {:.1}x",
                data_size as f64 / file_size as f64
            );

            if args.detailed {
                println!(
                    "   Format: {}",
                    args.image
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("unknown")
                );
            }
        },
        Err(e) => {
            println!("❌ Failed to load image: {}", e);
            error!("Failed to load image {:?}: {}", args.image, e);
            std::process::exit(1);
        },
    }

    Ok(())
}
