use eframe::egui::{self, Context, Key};
use ferrite_cache::CacheHandle;
use std::{path::PathBuf, sync::Arc};

use crate::{
    navigation::NavigationManager,
    ui::{menu::MenuBar, render::ImageRenderer, zoom::ZoomHandler},
};
use ferrite_config::FerriteConfig;

pub struct FeriteApp {
    config:        FerriteConfig,
    image_manager: ferrite_image::ImageManager,
    navigation:    NavigationManager,
    zoom_handler:  ZoomHandler,
    menu_bar:      MenuBar,
    cache_manager: Arc<CacheHandle>,
}
impl FeriteApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        initial_image: Option<PathBuf>,
        config: FerriteConfig,
        cache_manager: Arc<CacheHandle>,
    ) -> Self {
        let image_manager =
            ferrite_image::ImageManager::new(cache_manager.clone());
        let navigation = NavigationManager::new();
        let zoom_handler = ZoomHandler::new(config.zoom.default_zoom);
        let menu_bar = MenuBar::new(config.window.hide_menu);

        let mut app = Self {
            config,
            image_manager,
            navigation,
            zoom_handler,
            menu_bar,
            cache_manager,
        };

        if let Some(path) = initial_image {
            // First load the directory for navigation
            if let Some(()) = app.navigation.load_current_directory(&path) {
                tracing::info!("Successfully loaded directory for navigation");

                // Then load the initial image
                if let Ok(image_data) =
                    app.cache_manager.get_image(path.clone())
                {
                    // Set the image in the image manager
                    app.image_manager.current_image = Some(image_data);
                    app.image_manager.set_path(path);
                    tracing::info!("Successfully loaded initial image");
                } else {
                    tracing::error!("Failed to load initial image from cache");
                }
            }
        }

        app
    }
}

impl eframe::App for FeriteApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.navigation.handle_keyboard_input(
            ctx,
            &mut self.image_manager,
            &mut self.zoom_handler,
        );

        if ctx.input(|i| i.key_pressed(Key::M)) {
            self.menu_bar.toggle();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.menu_bar.is_hidden() {
                self.menu_bar.render(ui, ctx, &mut self.config);
            }

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
