use eframe::egui::{self, ColorImage, Pos2, Rect, TextureOptions, Ui};
use egui::{Color32, Context, Sense, Vec2};
use ferrite_config::{Corner, FeriteConfig};

use crate::{image::ImageManager, ui::zoom::ZoomHandler};

pub struct ImageRenderer;

impl ImageRenderer {
    // Main render function that now handles all interactions and redraws
    pub fn render(
        ui: &mut Ui,
        ctx: &Context,
        image_manager: &mut ImageManager,
        zoom_handler: &mut ZoomHandler,
        config: &FeriteConfig,
    ) -> bool {
        let mut needs_redraw = false;
        let panel_rect = ui.available_rect_before_wrap();

        // Handle texture creation/retrieval separately before other UI operations
        let texture_handle = if let Some(image_data) = image_manager.current_image() {
            if image_data.texture.is_none() {
                let size = [
                    image_data.original.width() as usize,
                    image_data.original.height() as usize,
                ];
                let image = image_data.original.to_rgba8();

                let texture = ctx.load_texture(
                    "current-image",
                    ColorImage::from_rgba_unmultiplied(size, image.as_flat_samples().as_slice()),
                    TextureOptions::LINEAR,
                );
                image_data.texture = Some(texture);
            }
            image_data.texture.as_ref()
        } else {
            None
        };

        if let Some(texture) = texture_handle {
            let original_size = texture.size_vec2();
            let scaled_size = original_size * zoom_handler.zoom_level();

            // Handle all input events and track if they require a redraw
            needs_redraw |= Self::handle_input(ctx, ui, zoom_handler, panel_rect);

            // Calculate image position and handle dragging
            let (image_rect, response) =
                Self::handle_image_positioning(ui, panel_rect, scaled_size, zoom_handler);

            // Update offset if dragged
            if response.dragged() {
                zoom_handler.add_offset(response.drag_delta());
                needs_redraw = true;
            }

            // Render the image
            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );

            // Render zoom indicator if enabled
            if config.zoom.show_zoom_level {
                Self::render_zoom_indicator(
                    ui,
                    zoom_handler,
                    panel_rect,
                    &config.zoom.zoom_display_corner,
                );
            }
        }

        needs_redraw
    }

    // Handle all input events in one place
    fn handle_input(
        ctx: &Context,
        ui: &Ui,
        zoom_handler: &mut ZoomHandler,
        panel_rect: Rect,
    ) -> bool {
        let mut needs_redraw = false;

        // Keyboard zoom controls
        if ctx.input(|i| i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)) {
            needs_redraw |= Self::handle_zoom(ui, zoom_handler, 1.0);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
            needs_redraw |= Self::handle_zoom(ui, zoom_handler, -1.0);
        }

        // Reset zoom and position
        if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
            zoom_handler.reset();
            needs_redraw = true;
        }

        // Scroll wheel zoom
        let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            needs_redraw |= Self::handle_zoom(ui, zoom_handler, scroll_delta);
        }

        needs_redraw
    }

    // Handle zoom operations and return whether a redraw is needed
    fn handle_zoom(ui: &Ui, zoom_handler: &mut ZoomHandler, scroll_delta: f32) -> bool {
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            let panel_rect = ui.available_rect_before_wrap();
            let old_center = panel_rect.center() + zoom_handler.offset();

            // Calculate zoom changes
            let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);

            // Update zoom maintaining mouse position
            let mouse_to_center = mouse_pos - old_center;
            let scale_factor = new_zoom / zoom_handler.zoom_level();
            let new_mouse_to_center = mouse_to_center * scale_factor;

            zoom_handler.set_zoom(new_zoom);
            zoom_handler.add_offset(mouse_to_center - new_mouse_to_center);

            true
        } else {
            false
        }
    }

    // Handle image positioning and dragging
    fn handle_image_positioning(
        ui: &mut Ui,
        panel_rect: Rect,
        scaled_size: Vec2,
        zoom_handler: &ZoomHandler,
    ) -> (Rect, egui::Response) {
        let image_rect =
            Rect::from_center_size(panel_rect.center() + zoom_handler.offset(), scaled_size);
        let response = ui.allocate_rect(image_rect, Sense::drag());
        (image_rect, response)
    }

    // Render zoom indicator overlay
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
