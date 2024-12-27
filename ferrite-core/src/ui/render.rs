use crate::ui::input;
use eframe::{
    egui::{self, ColorImage, Pos2, Rect, TextureOptions, Ui},
    Frame,
};
use egui::{Area, Color32, Context, FontFamily, Order, RichText, Sense, Vec2};
use ferrite_config::{Corner, FerriteConfig, IndicatorConfig};
use image::GenericImageView;

use crate::{image::ImageManager, ui::zoom::ZoomHandler};

use super::zoom::FitMode;

pub struct ImageRenderer;

impl ImageRenderer {
    pub fn render(
        ui: &mut Ui,
        ctx: &Context,
        image_manager: &mut ferrite_image::ImageManager,
        zoom_handler: &mut ZoomHandler,
        config: &FerriteConfig,
    ) {
        let panel_rect = ui.available_rect_before_wrap();

        // Handle keyboard input and general interactions
        input::handle_input(ctx, ui, zoom_handler, panel_rect);

        let current_image_size =
            if let Some(image_data) = image_manager.current_image.as_mut() {
                let (width, height) = image_data.dimensions();
                Some(Vec2::new(width as f32, height as f32))
            } else {
                None
            };

        // If we have an image, update zoom based on window size
        if let Some(image_size) = current_image_size {
            zoom_handler
                .update_for_window_resize(image_size, panel_rect.size());
        }

        // Handle texture creation/retrieval
        let texture_handle = if let Some(image_data) =
            image_manager.current_image.as_mut()
        {
            if image_manager.texture.is_none() {
                let size =
                    [image_data.width() as usize, image_data.height() as usize];
                let image = image_data.to_rgba8();

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
                image_manager.texture = Some(texture);
            }
            image_manager.texture.as_ref()
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

            // Handle zoom interactions
            let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                Self::handle_zoom(
                    ui,
                    zoom_handler,
                    scroll_delta.into(),
                    panel_rect,
                );
            }

            Self::render_zoom_indicator(ui, zoom_handler, &config.indicator);

            // Render the image
            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }

    /// Handles zoom operations with cursor-relative or center-based zooming
    fn handle_zoom(
        ui: &Ui,
        zoom_handler: &mut ZoomHandler,
        scroll_delta: f64,
        panel_rect: Rect,
    ) {
        // Calculate the zoom factor based on scroll direction
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);

        // Get the current image center accounting for panning
        let current_center = panel_rect.center() + zoom_handler.offset();

        if let Some(cursor_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Calculate the vector from cursor to current center
            let cursor_to_center = cursor_pos - current_center;

            // Calculate how this vector should scale with the new zoom
            let scale_factor = new_zoom / zoom_handler.zoom_level();
            let new_cursor_to_center = cursor_to_center * scale_factor as f32;

            // Calculate needed offset correction to maintain cursor position
            let offset_correction = cursor_to_center - new_cursor_to_center;

            // Apply the zoom and offset changes
            zoom_handler.set_zoom(new_zoom);
            zoom_handler.add_offset(offset_correction);
        } else {
            // If no cursor present, just zoom from center
            zoom_handler.set_zoom(new_zoom);
        }

        // Request a repaint for smooth updates
        ui.ctx().request_repaint();
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

    fn render_zoom_indicator(
        ui: &mut Ui,
        zoom_handler: &ZoomHandler,
        config: &IndicatorConfig,
    ) {
        // Only render if indicator should be shown
        if !config.show_percentage {
            return;
        }

        let percentage_text = format!("{:.0}%", zoom_handler.zoom_percentage());
        let screen_rect = ui.ctx().screen_rect();

        // Convert config padding to egui Vec2
        let padding =
            Vec2::new(config.padding.x() as f32, config.padding.y() as f32);

        // Calculate text size with some buffer for percentage characters
        let font_size = config.font_size as f32;
        let char_width = font_size * 0.6; // Approximate width per character
        let text_width = char_width * percentage_text.len() as f32;

        // Add margin for the frame
        let frame_margin = 8.0; // Inner margin of the frame
        let box_size = Vec2::new(
            text_width + frame_margin * 2.0,
            font_size + frame_margin * 2.0,
        );

        let corner_pos = match config.corner {
            Corner::TopLeft => Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.min.y + padding.y,
            ),
            Corner::TopRight => Pos2::new(
                screen_rect.max.x - box_size.x - padding.x,
                screen_rect.min.y + padding.y,
            ),
            Corner::BottomLeft => Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.max.y - box_size.y - padding.y,
            ),
            Corner::BottomRight => Pos2::new(
                screen_rect.max.x - box_size.x - padding.x,
                screen_rect.max.y - box_size.y - padding.y,
            ),
        };

        // Create a floating area for the indicator
        Area::new("zoom_indicator")
            .order(Order::Foreground)
            .fixed_pos(corner_pos)
            .show(ui.ctx(), |ui| {
                // Create a frame with background
                egui::Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(
                        config.background_color.r,
                        config.background_color.g,
                        config.background_color.b,
                        config.background_color.a,
                    ))
                    .rounding(4.0)
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        let rich_text = RichText::new(percentage_text)
                            .color(Color32::from_rgba_unmultiplied(
                                config.text_color.r,
                                config.text_color.g,
                                config.text_color.b,
                                config.text_color.a,
                            ))
                            .size(font_size)
                            .family(FontFamily::Proportional);

                        ui.label(rich_text);
                    });
            });
    }
}
