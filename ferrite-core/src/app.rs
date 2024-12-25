use eframe::egui::{self, Context, Key};
use ferrite_cache::{CacheConfig, CacheManager};
use std::{path::PathBuf, sync::Arc};
use tokio::runtime::Runtime;

use crate::{
    async_support::{handler::AsyncHandler, AsyncChannels},
    image::ImageManager,
    navigation::NavigationManager,
    ui::{menu::MenuBar, render::ImageRenderer, zoom::ZoomHandler},
};
use ferrite_config::FerriteConfig;

pub struct FeriteApp {
    config:         FerriteConfig,
    image_manager:  ImageManager,
    navigation:     NavigationManager,
    zoom_handler:   ZoomHandler,
    menu_bar:       MenuBar,
    async_channels: AsyncChannels,
    cache_manager:  Arc<CacheManager>,
}
impl FeriteApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        initial_image: Option<PathBuf>,
        config: FerriteConfig,
        runtime: Runtime,
    ) -> Self {
        let runtime = Arc::new(runtime);
        let cache_config = CacheConfig::default();
        let cache_manager = Arc::new(CacheManager::new(cache_config));

        let image_manager = ImageManager::new();
        let navigation = NavigationManager::new();
        let zoom_handler = ZoomHandler::new(config.zoom.default_zoom);
        let menu_bar = MenuBar::new(config.window.hide_menu);

        let (async_channels, request_rx, response_tx) = AsyncChannels::new(32);
        let async_handler = AsyncHandler::new(runtime, cache_manager.clone());
        async_handler.spawn_handler(request_rx, response_tx);

        let mut app = Self {
            config,
            image_manager,
            navigation,
            zoom_handler,
            menu_bar,
            async_channels,
            cache_manager,
        };

        if let Some(path) = initial_image {
            if let Some(()) = app.navigation.load_current_directory(&path) {
                tracing::info!("Successfully loaded directory for navigation");
                let _ = app.async_channels.request_tx.blocking_send(
                    crate::async_support::message::ImageRequest::Load(path),
                );
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
                    let _ = self.async_channels.request_tx.blocking_send(
                        crate::async_support::message::ImageRequest::Load(
                            path.clone(),
                        ),
                    );
                }
            }
        }
    }
}

impl eframe::App for FeriteApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(response) = self.async_channels.response_rx.try_recv() {
            match response {
                crate::async_support::message::ImageResponse::Loaded(
                    path,
                    image,
                ) => {
                    self.image_manager.set_image(image);
                    self.image_manager.set_path(path);
                },
                crate::async_support::message::ImageResponse::Error(_) => {
                    tracing::warn!("Failed to load image");
                },
            }
        }

        if ctx.input(|i| i.key_pressed(Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<_> = ctx
                .input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|f| f.path)
                .collect();
            self.handle_files_dropped(ctx, files);
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
