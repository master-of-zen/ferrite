use eframe::egui::{self, Context, Key};
use ferrite_cache::CacheHandle;
use std::{path::PathBuf, sync::Arc};
use tracing::instrument;

use ferrite_config::FerriteConfig;
use ferrite_navigation::NavigationManager;
use ferrite_ui::{HelpMenu, ImageRenderer, RenderResult, ZoomHandler};

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
        let delete_pressed =
            ctx.input(|i| i.key_pressed(self.config.controls.delete_key));

        if delete_pressed {
            self.handle_delete();
        } else if next_pressed {
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

    fn handle_delete(&mut self) {
        use tracing::{error, info};

        // Delete the current file
        match self.image_manager.delete_current_file() {
            Ok(Some(deleted_path)) => {
                info!("Successfully deleted file: {}", deleted_path.display());

                // Remove the deleted file from navigation and get next image
                if let Some(next_path) =
                    self.navigation.remove_deleted_file(&deleted_path)
                {
                    // Load the next image
                    if let Err(e) =
                        self.image_manager.load_image(next_path.clone())
                    {
                        error!(
                            "Failed to load next image after deletion: {}",
                            e
                        );
                    } else {
                        self.zoom_handler.reset_view_position();
                        self.cache_nearby_images();
                    }
                } else {
                    info!("No more images to display after deletion");
                }
            },
            Ok(None) => {
                info!("No file to delete");
            },
            Err(e) => {
                error!("Failed to delete file: {}", e);
            },
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
            let render_result: RenderResult = ImageRenderer::render(
                ui,
                ctx,
                &mut self.image_manager,
                &mut self.zoom_handler,
                &self.config,
                &self.config.controls,
            );

            // Handle delete button click
            if render_result.delete_requested {
                self.handle_delete();
            }

            self.help_menu.render(
                ui,
                &self.config.help_menu,
                &self.config.controls,
            );
        });
    }
}
