use crate::ui::input;
use eframe::egui::{self, ColorImage, Pos2, Rect, TextureOptions, Ui};
use egui::{Color32, Context, Sense, Vec2};
use ferrite_config::{Corner, FerriteConfig};

use crate::{image::ImageManager, ui::zoom::ZoomHandler};

use super::zoom::FitMode;

pub struct ImageRenderer;

impl ImageRenderer {
    pub fn render(
        ui: &mut Ui,
        ctx: &Context,
        image_manager: &mut ImageManager,
        zoom_handler: &mut ZoomHandler,
        config: &FerriteConfig,
    ) {
        let panel_rect = ui.available_rect_before_wrap();

        input::handle_input(ctx, ui, zoom_handler, panel_rect);

        // Handle texture creation/retrieval
        let texture_handle =
            if let Some(image_data) = image_manager.current_image() {
                if image_data.texture.is_none() {
                    let size = [
                        image_data.original.width() as usize,
                        image_data.original.height() as usize,
                    ];
                    let image = image_data.original.to_rgba8();

                    let texture = ctx.load_texture(
                        "current-image",
                        ColorImage::from_rgba_unmultiplied(
                            size,
                            image.as_flat_samples().as_slice(),
                        ),
                        TextureOptions::LINEAR,
                    );

                    // Update zoom for new image
                    let image_size = Vec2::new(size[0] as f32, size[1] as f32);
                    zoom_handler
                        .update_for_new_image(image_size, panel_rect.size());

                    image_data.texture = Some(texture);
                }
                image_data.texture.as_ref()
            } else {
                None
            };

        if let Some(texture) = texture_handle {
            let original_size = texture.size_vec2();
            let scaled_size = original_size * zoom_handler.zoom_level() as f32;

            // Handle image positioning and dragging
            let (image_rect, response) = Self::handle_image_positioning(
                ui,
                panel_rect,
                scaled_size,
                zoom_handler,
            );

            // Update offset if dragged
            if response.dragged() {
                zoom_handler.add_offset(response.drag_delta());
            }

            // Render the image
            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );

            Self::render_zoom_indicator(
                ui,
                zoom_handler,
                panel_rect,
                &config.indicator.corner,
            );
        }
    }

    fn handle_image_positioning(
        ui: &mut Ui,
        panel_rect: Rect,
        scaled_size: Vec2,
        zoom_handler: &ZoomHandler,
    ) -> (Rect, egui::Response) {
        // Calculate the center of the panel as our reference point
        let panel_center = panel_rect.center();

        // Apply the current pan offset from the zoom handler to our center
        // position
        let image_center = panel_center + zoom_handler.offset();

        // Create the initial image rectangle centered at our calculated
        // position
        let image_rect = Rect::from_center_size(image_center, scaled_size);

        // If we're in a fit mode, we might want to constrain the dragging
        let constrain_dragging = zoom_handler.get_fit_mode() != FitMode::Custom;

        // Create the interactive area for the image
        let response = ui.allocate_rect(image_rect, Sense::drag());

        // If we're constraining the drag and the image is being dragged
        let final_rect = if constrain_dragging && response.dragged() {
            // Get the drag delta from the response
            let drag_delta = response.drag_delta();

            // Calculate the proposed new position
            let mut new_rect = image_rect.translate(drag_delta);

            // Minimum pixels of image that must remain visible
            let min_visible = 50.0;

            // Constrain horizontally
            if new_rect.max.x < panel_rect.min.x + min_visible {
                new_rect = new_rect.translate(Vec2::new(
                    panel_rect.min.x + min_visible - new_rect.max.x,
                    0.0,
                ));
            }
            if new_rect.min.x > panel_rect.max.x - min_visible {
                new_rect = new_rect.translate(Vec2::new(
                    panel_rect.max.x - min_visible - new_rect.min.x,
                    0.0,
                ));
            }

            // Constrain vertically
            if new_rect.max.y < panel_rect.min.y + min_visible {
                new_rect = new_rect.translate(Vec2::new(
                    0.0,
                    panel_rect.min.y + min_visible - new_rect.max.y,
                ));
            }
            if new_rect.min.y > panel_rect.max.y - min_visible {
                new_rect = new_rect.translate(Vec2::new(
                    0.0,
                    panel_rect.max.y - min_visible - new_rect.min.y,
                ));
            }

            new_rect
        } else {
            image_rect
        };

        (final_rect, response)
    }

    // Handle all input events in one place
    fn handle_input(
        ctx: &Context,
        ui: &Ui,
        zoom_handler: &mut ZoomHandler,
        panel_rect: Rect,
    ) {
        // Keyboard zoom controls
        if ctx.input(|i| {
            i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)
        }) {
            Self::handle_zoom(ui, zoom_handler, 1.0);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
            Self::handle_zoom(ui, zoom_handler, -1.0);
        }

        // Scroll wheel zoom
        let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            Self::handle_zoom(ui, zoom_handler, scroll_delta.into());
        }

        // Reset zoom and position
        if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
            zoom_handler.reset();
        }
    }

    // Handle zoom operations and return whether a redraw is needed
    fn handle_zoom(ui: &Ui, zoom_handler: &mut ZoomHandler, scroll_delta: f64) {
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Calculate zoom factor - adjust for smoother zooming
            let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let new_zoom =
                (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);

            // Apply new zoom level
            zoom_handler.set_zoom(new_zoom);
        }
    }

    // Handle image positioning and dragging
    fn render_zoom_indicator(
        ui: &mut Ui,
        zoom_handler: &ZoomHandler,
        panel_rect: Rect,
        corner: &Corner,
    ) {
        let mode_text = match zoom_handler.get_fit_mode() {
            FitMode::OneToOne => "1:1",
            FitMode::FitLonger => "Fit",
            FitMode::FitShorter => "Fill",
            FitMode::Custom => {
                &format!("{:.0}%", zoom_handler.zoom_percentage())
            },
        };

        let text_size = Vec2::new(60.0, 20.0);
        let corner_pos = match corner {
            Corner::TopLeft => panel_rect.min + Vec2::new(5.0, 5.0),
            Corner::TopRight => {
                panel_rect.max - Vec2::new(text_size.x + 5.0, -5.0)
            },
            Corner::BottomLeft => {
                panel_rect.max - Vec2::new(-5.0, text_size.y + 5.0)
            },
            Corner::BottomRight => {
                panel_rect.max - Vec2::new(text_size.x + 5.0, text_size.y + 5.0)
            },
        };

        let text_rect = Rect::from_min_size(corner_pos, text_size);
        ui.put(text_rect, egui::Label::new(mode_text));
    }
}
