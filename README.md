# Ferrite

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
+ look up for your system how to make launch ferrite as default image viewer
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
- `Right Click + Drag`: Selection box
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
