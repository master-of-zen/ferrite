use eframe::egui;
use egui::*;
use image::DynamicImage;
use lru::LruCache;
use std::{collections::HashSet, path::PathBuf, process::exit, fs};
use tracing::{info, instrument, warn};

/// The main application state structure holds all the data needed for the image viewer
pub struct FeriteApp {
    // Image handling components
    image_cache: LruCache<PathBuf, DynamicImage>,
    current_image: Option<ImageData>,
    current_path: Option<PathBuf>,
    
    // Directory navigation components
    directory_images: Vec<PathBuf>,
    current_image_index: usize,
    loading_in_progress: HashSet<PathBuf>,  // Track which images are currently being loaded

    // UI state components
    zoom_level: f32,
    drag_offset: Vec2,
    show_performance: bool,
}

/// Helper structure that keeps together the original image data and its GPU texture
struct ImageData {
    texture: Option<egui::TextureHandle>,
    original: DynamicImage,
}

impl Default for FeriteApp {
    fn default() -> Self {
        Self {
            image_cache: LruCache::new(std::num::NonZeroUsize::new(5).unwrap()),
            current_image: None,
            current_path: None,
            directory_images: Vec::new(),
            current_image_index: 0,
            loading_in_progress: HashSet::new(),
            zoom_level: 1.0,
            drag_offset: Vec2::ZERO,
            show_performance: false,
        }
    }
}

impl FeriteApp {
    #[instrument(skip(cc))]
    pub fn new(cc: &eframe::CreationContext<'_>, initial_image: Option<PathBuf>) -> Self {
        info!("Initializing Ferrite");

        let mut fonts = FontDefinitions::default();
        cc.egui_ctx.set_fonts(fonts);

        let mut app = Self::default();

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

    /// Loads all supported image files from the directory containing the given path
    fn load_directory_images(&mut self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            if let Ok(entries) = fs::read_dir(parent) {
                self.directory_images = entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        
                        if path.is_file() {
                            if let Some(extension) = path.extension() {
                                if matches!(
                                    extension.to_str().map(|s| s.to_lowercase()),
                                    Some(ext) if ["jpg", "jpeg", "png", "gif", "bmp"].contains(&ext.as_str())
                                ) {
                                    return Some(path);
                                }
                            }
                        }
                        None
                    })
                    .collect();
                
                self.directory_images.sort();
                
                if let Some(current_path) = &self.current_path {
                    if let Some(index) = self.directory_images.iter().position(|p| p == current_path) {
                        self.current_image_index = index;
                    }
                }
            }
        }
    }

    /// Initiates loading of adjacent images to prepare for navigation
    fn preload_adjacent_images(&mut self) {
        if self.directory_images.is_empty() {
            return;
        }

        // Calculate indices for previous and next images
        let prev_index = if self.current_image_index == 0 {
            self.directory_images.len() - 1
        } else {
            self.current_image_index - 1
        };

        let next_index = (self.current_image_index + 1) % self.directory_images.len();

        // Get paths for adjacent images
        let prev_path = self.directory_images[prev_index].clone();
        let next_path = self.directory_images[next_index].clone();

        // Preload previous image if not already in cache or loading
        if !self.image_cache.contains(&prev_path) && !self.loading_in_progress.contains(&prev_path) {
            self.loading_in_progress.insert(prev_path.clone());
            self.load_image_async(prev_path);
        }

        // Preload next image if not already in cache or loading
        if !self.image_cache.contains(&next_path) && !self.loading_in_progress.contains(&next_path) {
            self.loading_in_progress.insert(next_path.clone());
            self.load_image_async(next_path);
        }
    }

    /// Loads an image asynchronously using rayon's thread pool
    fn load_image_async(&mut self, path: PathBuf) {
        use rayon::prelude::*;
        
        let ctx = egui::Context::default();
        
        // Spawn the loading task in a separate thread
        std::thread::spawn(move || {
            if let Ok(img) = image::open(&path) {
                // Request a UI update when the image is loaded
                ctx.request_repaint();
            }
        });
    }

    #[instrument(skip(self, path))]
    fn load_image(&mut self, path: PathBuf) {
        info!("Loading image: {:?}", path);

        // Check if the image is already in our cache
        if let Some(img) = self.image_cache.get(&path) {
            info!("Image found in cache");
            self.current_image = Some(ImageData {
                texture: None,
                original: img.clone(),
            });
            self.current_path = Some(path.clone());
            
            // Load directory contents if this is a new directory
            self.load_directory_images(&path);
            
            // Start preloading adjacent images
            self.preload_adjacent_images();
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
                self.current_path = Some(path.clone());
                
                // Load directory contents if this is a new directory
                self.load_directory_images(&path);
                
                // Reset view parameters
                self.zoom_level = 1.0;
                self.drag_offset = Vec2::ZERO;
                
                // Start preloading adjacent images
                self.preload_adjacent_images();
                
                // Remove from loading set if it was there
                self.loading_in_progress.remove(&path);
            }
            Err(e) => {
                warn!("Failed to load image: {}", e);
                // Remove from loading set on error
                self.loading_in_progress.remove(&path);
            }
        }
    }

    fn next_image(&mut self) {
        if !self.directory_images.is_empty() {
            self.current_image_index = (self.current_image_index + 1) % self.directory_images.len();
            let next_path = self.directory_images[self.current_image_index].clone();
            self.load_image(next_path);
        }
    }

    fn previous_image(&mut self) {
        if !self.directory_images.is_empty() {
            self.current_image_index = if self.current_image_index == 0 {
                self.directory_images.len() - 1
            } else {
                self.current_image_index - 1
            };
            let prev_path = self.directory_images[self.current_image_index].clone();
            self.load_image(prev_path);
        }
    }

    fn handle_files_dropped(&mut self, _ctx: &egui::Context, files: Vec<PathBuf>) {
        if let Some(path) = files.first() {
            if let Some(extension) = path.extension() {
                if matches!(
                    extension.to_str().map(|s| s.to_lowercase()),
                    Some(ext) if ["jpg", "jpeg", "png", "gif", "bmp"].contains(&ext.as_str())
                ) {
                    self.load_image(path.clone());
                }
            }
        }
    }


    // Helper method to convert zoom level to percentage for display
    fn zoom_percentage(&self) -> f32 {
        self.zoom_level * 100.0
    }

    // Implement smoother zoom handling with wider range
    fn handle_zoom(&mut self, ui: &Ui, scroll_delta: f32) {
        // Get current mouse position
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Calculate zoom center (mouse position)
            let panel_rect = ui.available_rect_before_wrap();
            let old_center = panel_rect.center() + self.drag_offset;
            
            // Calculate zoom factor - larger steps for faster zooming
            let zoom_step = if scroll_delta > 0.0 { 1.25 } else { 0.8 };
            let new_zoom = (self.zoom_level * zoom_step).clamp(0.1, 10.0);
            
            // Calculate relative position of mouse to image center
            let mouse_to_center = mouse_pos - old_center;
            
            // Scale the offset based on zoom change
            let scale_factor = new_zoom / self.zoom_level;
            let new_mouse_to_center = mouse_to_center * scale_factor;
            
            // Update the offset to maintain mouse position
            self.drag_offset += mouse_to_center - new_mouse_to_center;
            
            // Apply new zoom level
            self.zoom_level = new_zoom;
            
            // Request repaint for smooth updates
            ui.ctx().request_repaint();
        }
    }

    // Enhanced image rendering with proper scaling
    fn render_image(&mut self, ui: &mut Ui) {
        let panel_rect = ui.available_rect_before_wrap();
        
        if let Some(image_data) = &mut self.current_image {
            // Get or create texture
            let texture: &TextureHandle = match &image_data.texture {
                Some(texture) => texture,
                None => {
                    let size = [
                        image_data.original.width() as usize,
                        image_data.original.height() as usize,
                    ];
                    let image = image_data.original.to_rgba8();
                    
                    image_data.texture = Some(ui.ctx().load_texture(
                        "current-image",
                        ColorImage::from_rgba_unmultiplied(size, image.as_flat_samples().as_slice()),
                        TextureOptions::LINEAR,  // Use linear filtering for smoother scaling
                    ));
                    image_data.texture.as_ref().unwrap()
                }
            };

            // Calculate scaled dimensions while preserving aspect ratio
            let original_size = texture.size_vec2();
            let scaled_size = original_size * self.zoom_level;
            
            // Calculate image rectangle
            let image_rect = Rect::from_center_size(
                panel_rect.center() + self.drag_offset,
                scaled_size,
            );
            
            // Handle dragging
            let response = ui.allocate_rect(image_rect, Sense::drag());
            if response.dragged() {
                self.drag_offset += response.drag_delta();
            }
            
            // Draw the image
            let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
            ui.painter().image(
                texture.id(),
                image_rect,
                uv,
                Color32::WHITE,
            );
            
            // Display zoom percentage in corner
            let zoom_text = format!("{:.0}%", self.zoom_percentage());
            ui.put(
                Rect::from_min_max(panel_rect.min + vec2(5.0, 5.0), panel_rect.min + vec2(60.0, 25.0)),
                Label::new(RichText::new(zoom_text).monospace()),
            );
        }
    }
}


impl eframe::App for FeriteApp {
    #[instrument(skip(self, ctx, _frame))]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle file drops from the system
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<_> = ctx
                .input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|f| f.path)
                .collect();
            self.handle_files_dropped(ctx, files);
        }

        // Handle keyboard navigation between images
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D)) {
            self.next_image();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A)) {
            self.previous_image();
        }

        // Handle all zoom-related inputs
        if ctx.input(|i| i.modifiers.ctrl) {
            // Handle scroll wheel zooming
            let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.handle_zoom(ui, scroll_delta);
                });
            }
            
            // Handle keyboard zoom controls (plus/minus keys)
            if ctx.input(|i| i.key_pressed(Key::Equals) || i.key_pressed(Key::Plus)) {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.handle_zoom(ui, 10.0);  // Positive value for zoom in
                });
            }
            if ctx.input(|i| i.key_pressed(Key::Minus)) {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.handle_zoom(ui, -10.0);  // Negative value for zoom out
                });
            }
            
            // Reset zoom and position with Ctrl+0
            if ctx.input(|i| i.key_pressed(Key::Num0)) {
                self.zoom_level = 1.0;
                self.drag_offset = Vec2::ZERO;
                ctx.request_repaint();
            }
        }

        // Main UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                    if ui.button("Toggle Performance").clicked() {
                        self.show_performance = !self.show_performance;
                        ui.close_menu();
                    }
                });
                
                // Add View menu for zoom controls
                ui.menu_button("View", |ui| {
                    if ui.button("Zoom In (Ctrl++)").clicked() {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            self.handle_zoom(ui, 10.0);
                        });
                        ui.close_menu();
                    }
                    if ui.button("Zoom Out (Ctrl-)").clicked() {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            self.handle_zoom(ui, -10.0);
                        });
                        ui.close_menu();
                    }
                    if ui.button("Reset Zoom (Ctrl+0)").clicked() {
                        self.zoom_level = 1.0;
                        self.drag_offset = Vec2::ZERO;
                        ctx.request_repaint();
                        ui.close_menu();
                    }
                });
            });

            // Render the main image
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
                ui.label(format!("Zoom level: {:.1}%", self.zoom_percentage()));
                if let Some(path) = &self.current_path {
                    ui.label(format!(
                        "Current image: {:?}",
                        path.file_name().unwrap_or_default()
                    ));
                }
                ui.label(format!(
                    "Image position: {}/{}",
                    self.current_image_index + 1,
                    self.directory_images.len()
                ));
                ui.label(format!(
                    "Images loading: {}",
                    self.loading_in_progress.len()
                ));
            });
        }
    }
}