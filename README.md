# Ferrite
[![](https://img.shields.io/crates/v/ferrite.svg)](https://crates.io/crates/ferrite)
[![](https://tokei.rs/b1/github/master-of-zen/ferrite?category=code)](https://github.com/master-of-zen/ferrite)


IT VIEWS IMAGES FAST

## Features

- 🚀Blazingly-high-performance image loading with async caching
- ⚙️Highly Configurable
- 🎯Straight to the point
## ⚡Quick Start

```bash
cargo install ferrite
ferrite [IMAGE_PATH]
```
+ Set it as default image viewer.
[Here are instructions](/install/README.md)
## Keybindings

### Navigation
- `Left Arrow` / `A`: Previous image
- `Right Arrow` / `D`: Next image
- `Q`: Quit

### 🔍View Controls
- `+` / `=` / `W`: Zoom in
- `-` / `S`: Zoom out
- `0`: Reset zoom
- `F`: Toggle fit mode

### 🖱️Mouse Controls
- `Scroll`: Zoom in/out
- `Left Click + Drag`: Pan image

## ⚙️Configuration

Ferrite uses TOML for configuration. Generate a default config:

```bash
ferrite --generate-config
```

The config file location is determined by:
1. `FERRITE_CONF` environment variable
2. Default XDG config path (`~/.config/ferrite/config.toml`)

Fields in config are self descriptive.

## 🏃PERFORMANCE

* PARALLEL image loading
* SMART caching system
* ADAPTIVE memory usage
* ASYNC I/O everywhere
* GPU POWER via egui

## License

GPL-3.0-or-later
