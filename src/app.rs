use eframe::egui;
use egui::*;
use image::DynamicImage;
use lru::LruCache;
use std::{path::PathBuf, process::exit};
use tracing::{info, instrument, warn};

// The main application state structure holds all the data needed for the image viewer
pub struct FeriteApp {
    // Image handling components
    // LRU cache helps manage memory by keeping only the most recently used images
    image_cache: LruCache<PathBuf, DynamicImage>,
    // Current image being displayed, wrapped in Option since we might not have an image loaded
    current_image: Option<ImageData>,
    // Path to the current image, useful for displaying filename and handling reloads
    current_path: Option<PathBuf>,

    // UI state components
    // Zoom level affects how large the image appears (1.0 is actual size)
    zoom_level: f32,
    // Tracks how far the user has dragged the image from its center position
    drag_offset: Vec2,
    // Controls visibility of the performance monitoring window
    show_performance: bool,
}

// Helper structure that keeps together the original image data and its GPU texture
// The texture is optional because we create it lazily when first rendering
struct ImageData {
    texture: Option<egui::TextureHandle>,
    original: DynamicImage,
}

// Default implementation provides initial values for a new FeriteApp instance
impl Default for FeriteApp {
    fn default() -> Self {
        Self {
            // Initialize cache with capacity for 5 images
            image_cache: LruCache::new(std::num::NonZeroUsize::new(5).unwrap()),
            current_image: None,
            current_path: None,
            zoom_level: 1.0,
            drag_offset: Vec2::ZERO,
            show_performance: false,
        }
    }
}

impl FeriteApp {
    // Creates a new instance of the application
    // The #[instrument] attribute enables tracing for performance monitoring
    #[instrument(skip(cc))]
    pub fn new(cc: &eframe::CreationContext<'_>, initial_image: Option<PathBuf>) -> Self {
        info!("Initializing Ferrite");

        // Set up custom fonts if needed
        let mut fonts = FontDefinitions::default();
        // Add custom fonts here if desired
        cc.egui_ctx.set_fonts(fonts);

        // Create the application instance
        let mut app = Self::default();

        // If an initial image was provided via command line, load it
        if let Some(path) = initial_image {
            info!("Loading initial image from command line: {:?}", path);
            if path.exists() {
                app.load_image(path);
            } else {
                warn!("Initial image path does not exist: {:?}", path);
            }
        }

        app
    }

    // Handles loading a new image from a path
    // The image is stored both in the cache and set as the current image
    #[instrument(skip(self, path))]
    fn load_image(&mut self, path: PathBuf) {
        info!("Loading image: {:?}", path);

        // Check if the image is already in our cache
        if let Some(img) = self.image_cache.get(&path) {
            info!("Image found in cache");
            self.current_image = Some(ImageData {
                texture: None, // Texture will be created on next frame
                original: img.clone(),
            });
            self.current_path = Some(path);
            return;
        }

        // If not in cache, load the new image from disk
        match image::open(&path) {
            Ok(img) => {
                info!("Image loaded successfully");
                self.image_cache.put(path.clone(), img.clone());
                self.current_image = Some(ImageData {
                    texture: None,
                    original: img,
                });
                self.current_path = Some(path);
                // Reset view parameters when loading a new image
                self.zoom_level = 1.0;
                self.drag_offset = Vec2::ZERO;
            }
            Err(e) => {
                warn!("Failed to load image: {}", e);
            }
        }
    }

    // Handles files being dropped onto the application window
    fn handle_files_dropped(&mut self, _ctx: &egui::Context, files: Vec<PathBuf>) {
        if let Some(path) = files.first() {
            if let Some(extension) = path.extension() {
                // Check if the file has a supported image extension
                if matches!(
                    extension.to_str().map(|s| s.to_lowercase()),
                    Some(ext) if ["jpg", "jpeg", "png", "gif", "bmp"].contains(&ext.as_str())
                ) {
                    self.load_image(path.clone());
                }
            }
        }
    }

    // Renders the current image to the UI, handling zoom and pan
    fn render_image(&mut self, ui: &mut Ui) {
        let available_size = ui.available_size();

        if let Some(image_data) = &mut self.current_image {
            // Create or get the texture for rendering
            let texture: &egui::TextureHandle = match &image_data.texture {
                Some(texture) => texture,
                None => {
                    // Convert image data to a format egui can display
                    let size = [
                        image_data.original.width() as usize,
                        image_data.original.height() as usize,
                    ];
                    let image = image_data.original.to_rgba8();
                    let pixels = image.as_flat_samples();

                    // Create the GPU texture from our image data
                    image_data.texture = Some(ui.ctx().load_texture(
                        "current-image",
                        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                        Default::default(),
                    ));
                    image_data.texture.as_ref().unwrap()
                }
            };

            // Calculate display size while maintaining aspect ratio
            let aspect_ratio = texture.size_vec2().x / texture.size_vec2().y;
            let mut size = texture.size_vec2() * self.zoom_level;

            // Ensure the image fits within the available space
            if size.x > available_size.x {
                size.x = available_size.x;
                size.y = size.x / aspect_ratio;
            }
            if size.y > available_size.y {
                size.y = available_size.y;
                size.x = size.y * aspect_ratio;
            }

            // Position the image in the center of the available space
            let image_rect = Rect::from_center_size(
                ui.available_rect_before_wrap().center() + self.drag_offset,
                size,
            );

            // Handle dragging the image around
            let response = ui.allocate_rect(image_rect, Sense::drag());
            if response.dragged() {
                self.drag_offset += response.drag_delta();
            }

            // Draw the actual image
            ui.put(image_rect, egui::Image::new(texture));
        }
    }
}

// Implementation of the eframe::App trait which provides the main update loop
impl eframe::App for FeriteApp {
    #[instrument(skip(self, ctx, _frame))]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle file drops from the operating system
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<_> = ctx
                .input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|f| f.path)
                .collect();
            self.handle_files_dropped(ctx, files);
        }

        // Handle zooming with Ctrl + Mouse Wheel
        ctx.input(|i| {
            if i.modifiers.ctrl {
                self.zoom_level *= 1.0 - (i.raw_scroll_delta.y / 1000.0);
                self.zoom_level = self.zoom_level.clamp(0.1, 10.0);
            }
        });

        // Main UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        // TODO: Implement file dialog
                    }
                    if ui.button("Toggle Performance").clicked() {
                        self.show_performance = !self.show_performance;
                    }
                });
            });

            // Image display
            self.render_image(ui);
        });

        // Performance monitoring window
        if self.show_performance {
            egui::Window::new("Performance").show(ctx, |ui| {
                ui.label(format!(
                    "Cache size: {}/{}",
                    self.image_cache.len(),
                    self.image_cache.cap()
                ));
                ui.label(format!("Zoom level: {:.2}x", self.zoom_level));
                if let Some(path) = &self.current_path {
                    ui.label(format!(
                        "Current image: {:?}",
                        path.file_name().unwrap_or_default()
                    ));
                }
            });
        }
        exit(0);
    }
}
