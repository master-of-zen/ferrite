use crate::ui::zoom::ZoomHandler;
use eframe::egui::{self, Context, Rect, Ui};
use egui::{Pos2, Vec2};

pub fn handle_input(
    ctx: &Context,
    ui: &Ui,
    zoom_handler: &mut ZoomHandler,
    panel_rect: Rect,
) {
    // Keyboard zoom controls
    if ctx.input(|i| {
        i.key_pressed(egui::Key::Equals)
            || i.key_pressed(egui::Key::Plus)
            || i.key_pressed(egui::Key::W)
    }) {
        handle_zoom(ui, zoom_handler, 1.0);
    }
    if ctx.input(|i| {
        i.key_pressed(egui::Key::Minus) || i.key_pressed(egui::Key::S)
    }) {
        handle_zoom(ui, zoom_handler, -1.0);
    }

    // Scroll wheel zoom
    let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
    if scroll_delta != 0.0 {
        handle_zoom(ui, zoom_handler, scroll_delta.into());
    }

    // Reset zoom and position
    if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
        zoom_handler.reset();
    }
}

fn handle_zoom(ui: &Ui, zoom_handler: &mut ZoomHandler, scroll_delta: f64) {
    // Only handle zoom if we have a cursor position
    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
        // Calculate the current center of the image
        let panel_rect = ui.available_rect_before_wrap();
        let old_center = panel_rect.center() + zoom_handler.offset();

        // Calculate new zoom level with smooth stepping
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);

        // Find the vector from cursor to image center that we want to preserve
        let mouse_to_center = mouse_pos - old_center;

        // Calculate how this vector should scale with the new zoom level
        let scale_factor = new_zoom / zoom_handler.zoom_level();
        let new_mouse_to_center = mouse_to_center * scale_factor as f32;

        // Calculate and apply the offset needed to maintain cursor position
        let offset_correction = mouse_to_center - new_mouse_to_center;
        zoom_handler.add_offset(offset_correction);

        // Finally, apply the new zoom level
        zoom_handler.set_zoom(new_zoom);

        // Request repaint for smooth updates
        ui.ctx().request_repaint();
    } else {
        // If no cursor position (e.g. keyboard zoom), zoom from center
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);
        zoom_handler.set_zoom(new_zoom);
    }
}
