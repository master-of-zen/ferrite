# Ferrite CLI

Command-line interface for the Ferrite image viewer.

## Configuration

Ferrite uses a TOML configuration file that can be located in one of two ways:

1. Environment variable: `FERRITE_CONF=/path/to/config.toml`
2. Default system location:
   - Linux: ~/.config/ferrite/config.toml
   - macOS: ~/Library/Application Support/ferrite/config.toml
   - Windows: %APPDATA%\ferrite\config.toml

### Setting Up Configuration

Generate a default configuration file:
```bash
ferrite --generate-config