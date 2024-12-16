use eframe::egui::{self, ColorImage, Pos2, Rect, TextureOptions, Ui};
use egui::{Color32, Sense, Vec2};
use ferrite_config::{Corner, FeriteConfig};

use crate::{image::ImageManager, ui::zoom::ZoomHandler};

pub struct ImageRenderer;

impl ImageRenderer {
    pub fn render(
        ui: &mut Ui,
        image_manager: &mut ImageManager,
        zoom_handler: &mut ZoomHandler, // Changed to mutable reference
        config: &FeriteConfig,
    ) {
        let panel_rect = ui.available_rect_before_wrap();

        if let Some(image_data) = image_manager.current_image() {
            // Get or create texture
            let texture = match &image_data.texture {
                Some(texture) => texture,
                None => {
                    let size = [
                        image_data.original.width() as usize,
                        image_data.original.height() as usize,
                    ];
                    let image = image_data.original.to_rgba8();

                    image_data.texture = Some(ui.ctx().load_texture(
                        "current-image",
                        ColorImage::from_rgba_unmultiplied(
                            size,
                            image.as_flat_samples().as_slice(),
                        ),
                        TextureOptions::LINEAR,
                    ));
                    image_data.texture.as_ref().unwrap()
                }
            };

            let original_size = texture.size_vec2();
            let scaled_size = original_size * zoom_handler.zoom_level();

            let image_rect =
                Rect::from_center_size(panel_rect.center() + zoom_handler.offset(), scaled_size);

            let response = ui.allocate_rect(image_rect, Sense::drag());
            if response.dragged() {
                zoom_handler.add_offset(response.drag_delta());
            }

            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );

            if config.zoom.show_zoom_level {
                Self::render_zoom_indicator(
                    ui,
                    zoom_handler,
                    panel_rect,
                    &config.zoom.zoom_display_corner,
                );
            }
        }
    }

    fn render_zoom_indicator(
        ui: &mut Ui,
        zoom_handler: &ZoomHandler,
        panel_rect: Rect,
        corner: &Corner,
    ) {
        let zoom_text = format!("{:.0}%", zoom_handler.zoom_percentage());
        let text_size = Vec2::new(60.0, 20.0);

        let corner_pos = match corner {
            Corner::TopLeft => panel_rect.min + Vec2::new(5.0, 5.0),
            Corner::TopRight => panel_rect.max - Vec2::new(text_size.x + 5.0, -5.0),
            Corner::BottomLeft => panel_rect.max - Vec2::new(-5.0, text_size.y + 5.0),
            Corner::BottomRight => panel_rect.max - Vec2::new(text_size.x + 5.0, text_size.y + 5.0),
        };

        let text_rect = Rect::from_min_size(corner_pos, text_size);
        ui.put(text_rect, egui::Label::new(zoom_text));
    }
}
