# Ferrite CLI

Command-line interface handling for the Ferrite image viewer.

## Features
- Command-line argument parsing using clap
- Configuration file generation and management
- Log level configuration
- UI preferences customization

## Usage

```rust
use ferrite_cli::Args;
use ferrite_config::FeriteConfig;

let args = Args::parse();
let mut config = args.handle_config().unwrap_or_default();
args.apply_to_config(&mut config);

// Access parsed arguments
if let Some(image_path) = args.image_path {
    // Handle initial image
}
```