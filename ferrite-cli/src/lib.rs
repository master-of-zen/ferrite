use anyhow::Result;
use clap::Parser;
use ferrite_config::FerriteConfig;
use ferrite_logging::LogLevel;
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Ferrite - A fast and efficient image viewer"
)]
pub struct Args {
    /// Initial image file to open
    #[arg(value_name = "IMAGE")]
    pub image_path: Option<PathBuf>,

    /// Set the logging level
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: Option<String>,

    /// Generate a default configuration file
    #[arg(long)]
    pub generate_config: bool,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    pub fn handle_config(&self) -> Result<FerriteConfig> {
        if self.generate_config {
            let config_path = FerriteConfig::resolve_config_path()?;
            println!(
                "Generating default configuration at: {}",
                config_path.display()
            );
            let config = FerriteConfig::default();
            config.save_to_path(&config_path)?;

            // Print helpful information about configuration
            println!("\nConfiguration can be customized by:");
            println!(
                "1. Editing the file directly at: {}",
                config_path.display()
            );
            println!(
                "2. Setting FERRITE_CONF environment variable for a different \
                 location"
            );
            println!("\nExample environment variable usage:");
            println!(
                "export FERRITE_CONF=$HOME/.config/ferrite/custom-config.toml"
            );

            std::process::exit(0);
        }

        // Load configuration with environment awareness
        Ok(FerriteConfig::load()?)
    }

    /// Prints information about the current configuration path resolution
    pub fn print_config_info(&self) -> Result<()> {
        let config_path = FerriteConfig::resolve_config_path()?;

        println!("\nFerrite Configuration");
        println!("--------------------");
        println!("Current config path: {}", config_path.display());
        println!("\nConfiguration path is determined by:");
        println!("1. FERRITE_CONF environment variable (if set)");
        println!("2. Default system-specific location");

        if let Ok(env_path) = env::var("FERRITE_CONF") {
            println!("\nFERRITE_CONF is currently set to: {}", env_path);
        }

        let default_path = FerriteConfig::get_default_path()?;
        println!("Default path: {}", default_path.display());

        Ok(())
    }

    pub fn get_log_level(&self) -> LogLevel {
        self.log_level
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(LogLevel::Info)
    }
}
