use clap::Parser;
use ferrite_cache::{CacheConfig, CacheManager};
use once_cell::sync::OnceCell;
use std::{path::PathBuf, sync::Arc};
use tokio;
use tracing::{error, info};

static CACHE_MANAGER: OnceCell<Arc<CacheManager>> = OnceCell::new();

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
struct CacheState {
    entries:  Vec<PathBuf>,
    lru_list: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting directory scanner...");
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
            args.max_images
        );
        Arc::new(CacheManager::new(CacheConfig {
            max_image_count: args.max_images,
            thread_count:    4,
        }))
    });

    println!("▶️ Starting async execution");
    runtime_arc.block_on(async {
        run_directory_scan(args, cache_manager.clone()).await
    })?;

    println!("✨ Execution completed successfully");
    Ok(())
}

async fn run_directory_scan(
    args: Args,
    cache_manager: Arc<CacheManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.directory.is_dir() {
        println!("❌ Not a valid directory: {:?}", args.directory);
        error!("Not a valid directory: {:?}", args.directory);
        std::process::exit(1);
    }

    println!("📂 Scanning directory: {:?}", args.directory);
    let mut dir_entries = tokio::fs::read_dir(&args.directory).await?;
    let mut processed_count = 0;
    let mut success_count = 0;
    let mut tasks = Vec::new();

    let mut state = CacheState {
        entries: Vec::new(), lru_list: Vec::new()
    };

    println!("🔍 Looking for image files...");
    while let Some(entry) = dir_entries.next_entry().await? {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if ["jpg", "jpeg", "png", "gif"]
                    .contains(&ext_str.to_lowercase().as_str())
                {
                    processed_count += 1;
                    println!("📄 Found image {}: {:?}", processed_count, path);
                    state.lru_list.push(path.clone());

                    if args.verbose {
                        info!("Queueing image file: {:?}", path);
                    }

                    let cache_manager = cache_manager.clone();
                    let path_clone = path.clone();
                    let verbose = args.verbose;

                    let task = tokio::spawn(async move {
                        println!("⏳ Processing: {:?}", path_clone);
                        let start_time = std::time::Instant::now();
                        match cache_manager.get_image(path_clone.clone()).await
                        {
                            Ok(image_data) => {
                                let duration = start_time.elapsed();
                                let dims = image_data.dimensions();
                                let memory_size = image_data.data().len();
                                println!("✅ Cached: {:?}", path_clone);
                                println!(
                                    "   Dimensions: {}x{}",
                                    dims.0, dims.1
                                );
                                println!(
                                    "   Memory size: {} bytes",
                                    memory_size
                                );
                                println!(
                                    "   Raw file size: {} bytes",
                                    path_clone
                                        .metadata()
                                        .map(|m| m.len())
                                        .unwrap_or(0)
                                );
                                println!(
                                    "   Total processing time: {:?}",
                                    duration
                                );
                                if verbose {
                                    info!(
                                        "Cached image: {:?} ({}x{})",
                                        path_clone, dims.0, dims.1
                                    );
                                }
                                Ok(())
                            },
                            Err(e) => {
                                println!(
                                    "❌ Failed to cache: {:?}",
                                    path_clone
                                );
                                error!(
                                    "Failed to cache image {:?}: {}",
                                    path_clone, e
                                );
                                Err(())
                            },
                        }
                    });

                    tasks.push(task);
                }
            }
        }
    }

    println!("⏳ Waiting for all tasks to complete...");
    for task in tasks {
        if task.await?.is_ok() {
            success_count += 1;
        }
    }

    println!("📊 Summary of first pass:");
    println!("  - Images processed: {}", processed_count);
    println!("  - Successfully cached: {}", success_count);
    println!("  - Failed: {}", processed_count - success_count);
    println!(
        "  - Success rate: {:.1}%",
        (success_count as f64 / processed_count as f64) * 100.0
    );

    // Second pass - test cache
    println!("\n🔄 Testing cache by loading all images again...");
    let cache_start = std::time::Instant::now();
    let mut cached_tasks = Vec::new();

    for path in state.lru_list.iter() {
        let cache_manager = cache_manager.clone();
        let path_clone = path.clone();

        let task = tokio::spawn(async move {
            let start = std::time::Instant::now();
            match cache_manager.get_image(path_clone.clone()).await {
                Ok(_) => {
                    println!(
                        "✅ Cache hit: {:?} (took {:?})",
                        path_clone,
                        start.elapsed()
                    );
                    Ok(())
                },
                Err(e) => {
                    println!("❌ Cache miss: {:?} - {}", path_clone, e);
                    Err(())
                },
            }
        });
        cached_tasks.push(task);
    }

    let mut cache_success = 0;
    for task in cached_tasks {
        if task.await?.is_ok() {
            cache_success += 1;
        }
    }

    let cache_duration = cache_start.elapsed();
    println!("\n📊 Cache test results:");
    println!("  - Total images tested: {}", processed_count);
    println!("  - Successful cache hits: {}", cache_success);
    println!("  - Failed cache hits: {}", processed_count - cache_success);
    println!("  - Total cache test time: {:?}", cache_duration);
    println!(
        "  - Average time per image: {:?}",
        cache_duration.div_f64(processed_count as f64)
    );

    if args.verbose {
        let total_memory = cache_manager
            .get_total_memory_usage()
            .await
            .unwrap_or(0);
        println!("  - Total memory usage: {} bytes", total_memory);
    }

    info!(
        "Processed {} images ({} successful, {} failed)",
        processed_count,
        success_count,
        processed_count - success_count
    );

    Ok(())
}
