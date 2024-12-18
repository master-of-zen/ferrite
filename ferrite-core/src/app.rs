use eframe::egui::{self, Context, Key};
use std::path::PathBuf;

use crate::{
    image::{ImageLoadError, ImageManager},
    navigation::NavigationManager,
    ui::{menu::MenuBar, render::ImageRenderer, zoom::ZoomHandler},
};
use ferrite_config::FerriteConfig;

pub struct FeriteApp {
    config:        FerriteConfig,
    image_manager: ImageManager,
    navigation:    NavigationManager,
    zoom_handler:  ZoomHandler,
    menu_bar:      MenuBar,
}

impl FeriteApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        initial_image: Option<PathBuf>,
        config: FerriteConfig,
    ) -> Self {
        // Initialize our core components with their default states
        let image_manager = ImageManager::new();
        let navigation = NavigationManager::new();
        let zoom_handler = ZoomHandler::new(
            config.zoom.default_zoom, // Initial zoom level from config
        );
        let menu_bar = MenuBar::new(config.window.hide_menu);

        let mut app = Self {
            config,
            image_manager,
            navigation,
            zoom_handler,
            menu_bar,
        };

        if let Some(path) = initial_image {
            // First try to load the directory containing the image
            if let Some(()) = app.navigation.load_current_directory(&path) {
                tracing::info!("Successfully loaded directory for navigation");
            } else {
                tracing::warn!(
                    "Failed to load directory. Navigation between images will \
                     not be available"
                );
            }

            // Then attempt to load the initial image
            if !app.image_manager.load_image(path).is_ok() {
                tracing::warn!("Failed to load initial image");
            }
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
                    let _ = self.image_manager.load_image(path.clone());
                }
            }
        }
    }
}

impl eframe::App for FeriteApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle quit action by sending a close event to the application
        // context
        if ctx.input(|i| i.key_pressed(Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        // Handle file drops
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<_> = ctx
                .input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|f| f.path)
                .collect();
            self.handle_files_dropped(ctx, files);
        }

        // Handle navigation keyboard events
        self.navigation.handle_keyboard_input(
            ctx,
            &mut self.image_manager,
            &mut self.zoom_handler,
        );

        // Toggle menu visibility
        if ctx.input(|i| i.key_pressed(Key::M)) {
            self.menu_bar.toggle();
        }

        // Set up the main UI panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render menu bar if not hidden
            if !self.menu_bar.is_hidden() {
                self.menu_bar.render(ui, ctx, &mut self.config);
            }

            // Render the image and handle all interactions
            ImageRenderer::render(
                ui,
                ctx,
                &mut self.image_manager,
                &mut self.zoom_handler,
                &self.config,
            );
        });
    }
}
