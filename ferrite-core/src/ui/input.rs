use crate::ui::zoom::ZoomHandler;
use eframe::egui::{self, Context, Key, Rect, Ui};

pub fn handle_input(
    ctx: &Context,
    ui: &Ui,
    zoom_handler: &mut ZoomHandler,
    _panel_rect: Rect,
) {
    if ctx.input(|i| i.key_pressed(Key::F)) {
        zoom_handler.reset_to_default_fit_mode();
    }

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

    let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
    if scroll_delta != 0.0 {
        handle_zoom(ui, zoom_handler, scroll_delta.into());
    }

    if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
        zoom_handler.reset();
    }
}

fn handle_zoom(ui: &Ui, zoom_handler: &mut ZoomHandler, scroll_delta: f64) {
    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
        let panel_rect = ui.available_rect_before_wrap();
        let old_center = panel_rect.center() + zoom_handler.offset();

        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);

        let mouse_to_center = mouse_pos - old_center;

        let scale_factor = new_zoom / zoom_handler.zoom_level();
        let new_mouse_to_center = mouse_to_center * scale_factor as f32;

        let offset_correction = mouse_to_center - new_mouse_to_center;
        zoom_handler.add_offset(offset_correction);

        zoom_handler.set_zoom(new_zoom);

        ui.ctx().request_repaint();
    } else {
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);
        zoom_handler.set_zoom(new_zoom);
    }
}
