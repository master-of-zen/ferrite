use eframe::egui::{self, Context, Key};
use std::path::PathBuf;

use crate::{
    image::ImageManager,
    navigation::NavigationManager,
    ui::{self, menu::MenuBar, render::ImageRenderer, zoom::ZoomHandler},
};
use ferrite_config::FeriteConfig;

pub struct FeriteApp {
    config: FeriteConfig,
    image_manager: ImageManager,
    navigation: NavigationManager,
    zoom_handler: ZoomHandler,
    menu_bar: MenuBar,
}

impl FeriteApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        initial_image: Option<PathBuf>,
        config: FeriteConfig,
    ) -> Self {
        let image_manager = ImageManager::new(config.cache_size);
        let navigation = NavigationManager::new();
        let zoom_handler = ZoomHandler::new(config.default_zoom);
        let menu_bar = MenuBar::new(config.window.hide_menu);

        let mut app = Self {
            config,
            image_manager,
            navigation,
            zoom_handler,
            menu_bar,
        };

        if let Some(path) = initial_image {
            app.image_manager.load_image(path);
        }

        app
    }

    fn handle_files_dropped(&mut self, ctx: &Context, files: Vec<PathBuf>) {
        if let Some(path) = files.first() {
            if let Some(extension) = path.extension() {
                if matches!(
                    extension.to_str().map(|s| s.to_lowercase()),
                    Some(ext) if ["jpg", "jpeg", "png", "gif", "bmp"].contains(&ext.as_str())
                ) {
                    self.image_manager.load_image(path.clone());
                }
            }
        }
    }
}

impl eframe::App for FeriteApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle file drops by collecting dropped file paths and processing them
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<_> = ctx
                .input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|f| f.path)
                .collect();
            self.handle_files_dropped(ctx, files);
        }

        // Handle navigation keyboard events for moving between images
        self.navigation
            .handle_keyboard_input(ctx, &mut self.image_manager);

        // Toggle menu visibility on 'M' key press
        if ctx.input(|i| i.key_pressed(Key::M)) {
            self.menu_bar.toggle();
            // Request a repaint to immediately reflect the menu visibility change
            ctx.request_repaint();
        }

        // Set up the main UI panel that contains our image viewer
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render menu bar if it's not hidden
            if !self.menu_bar.is_hidden() {
                self.menu_bar.render(ui, ctx, &mut self.config);
            }

            // Render the image along with zoom controls
            // The ImageRenderer now handles both display and zoom interactions
            // This includes keyboard and mouse wheel zoom operations
            ImageRenderer::render(
                ui,
                &mut self.image_manager,
                &mut self.zoom_handler,
                &self.config,
            );
        });

        // Show performance metrics window if enabled in config
        if self.config.show_performance {
            self.image_manager.show_performance_window(ctx);
        }
    }
}
