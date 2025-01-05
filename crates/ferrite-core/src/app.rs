use eframe::egui::{self, Context, Key};
use ferrite_cache::CacheHandle;
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

use ferrite_config::FerriteConfig;
use ferrite_navigation::NavigationManager;
use ferrite_ui::{HelpMenu, ImageRenderer, ZoomHandler};

pub struct FeriteApp {
    config:        FerriteConfig,
    image_manager: ferrite_image::ImageManager,
    navigation:    NavigationManager,
    zoom_handler:  ZoomHandler,
    cache_manager: Arc<CacheHandle>,
    help_menu:     HelpMenu,
}

impl FeriteApp {
    #[instrument(skip(_cc, config, cache_manager), fields(initial_image = ?initial_image))]
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

        let mut app = Self {
            config,
            image_manager,
            navigation,
            zoom_handler,
            cache_manager,
            help_menu: HelpMenu::new(),
        };

        if let Some(path) = initial_image {
            if let Ok(()) = app.navigation.load_current_directory(&path) {
                if let Ok(image_data) =
                    app.cache_manager.get_image(path.clone())
                {
                    app.image_manager.current_image = Some(image_data);
                    app.image_manager.set_path(path);
                    app.cache_nearby_images();
                }
            }
        }

        app
    }

    fn cache_nearby_images(&self) {
        use tracing::{error, info};
        let cache_count = self.config.cache.preload_count; // Cache 2 images in each direction
        let (prev_paths, next_paths) =
            self.navigation.get_nearby_paths(cache_count);

        // Cache previous images
        for path in prev_paths {
            if let Err(e) = self.cache_manager.cache_image(path.clone()) {
                error!(
                    "Failed to cache previous image {}: {}",
                    path.display(),
                    e
                );
            } else {
                info!("Successfully cached previous image: {}", path.display());
            }
        }

        // Cache next images
        for path in next_paths {
            if let Err(e) = self.cache_manager.cache_image(path.clone()) {
                error!("Failed to cache next image {}: {}", path.display(), e);
            } else {
                info!("Successfully cached next image: {}", path.display());
            }
        }
    }

    fn handle_navigation(&mut self, ctx: &Context) {
        let next_pressed = ctx
            .input(|i| i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::D));
        let prev_pressed = ctx
            .input(|i| i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::A));

        if next_pressed {
            if let Some(next_path) = self.navigation.next_image() {
                let _ = self.image_manager.load_image(next_path);
                self.zoom_handler.reset_view_position();
                self.cache_nearby_images();
            }
        } else if prev_pressed {
            if let Some(prev_path) = self.navigation.previous_image() {
                let _ = self.image_manager.load_image(prev_path);
                self.zoom_handler.reset_view_position();
                self.cache_nearby_images();
            }
        }
    }
}

impl eframe::App for FeriteApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(self.config.controls.help_key)) {
            self.help_menu.toggle();
        }

        if ctx.input(|i| i.key_pressed(self.config.controls.quit_key)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.handle_navigation(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ImageRenderer::render(
                ui,
                ctx,
                &mut self.image_manager,
                &mut self.zoom_handler,
                &self.config,
                &self.config.controls,
            );
            self.help_menu.render(
                ui,
                &self.config.help_menu,
                &self.config.controls,
            );
        });
    }
}
