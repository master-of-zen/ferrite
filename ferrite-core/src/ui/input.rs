use crate::ui::zoom::ZoomHandler;
use eframe::egui::{self, Context, Rect, Ui};

pub fn handle_input(
    ctx: &Context,
    ui: &Ui,
    zoom_handler: &mut ZoomHandler,
    panel_rect: Rect,
) {
    // Keyboard zoom controls
    if ctx.input(|i| {
        i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)
    }) {
        handle_zoom(ui, zoom_handler, 1.0);
    }
    if ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
        handle_zoom(ui, zoom_handler, -1.0);
    }

    // Scroll wheel zoom
    let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
    if scroll_delta != 0.0 {
        handle_zoom(ui, zoom_handler, scroll_delta);
    }

    // Reset zoom and position
    if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
        zoom_handler.reset();
    }
}

fn handle_zoom(ui: &Ui, zoom_handler: &mut ZoomHandler, scroll_delta: f64) {
    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);
        zoom_handler.set_zoom(new_zoom);
    }
}
