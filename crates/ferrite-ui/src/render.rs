use std::path::PathBuf;

use crate::{input, FitMode, ZoomHandler};
use eframe::egui::{self, ColorImage, Pos2, Rect, TextureOptions, Ui};
use egui::{Area, Color32, Context, FontFamily, Order, RichText, Sense, Vec2};
use ferrite_config::{FerriteConfig, IndicatorConfig, Position};
use image::GenericImageView;
use tracing::instrument;

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
        input::handle_input(ctx, ui, zoom_handler, panel_rect);

        let current_image_size =
            image_manager
                .current_image
                .as_mut()
                .map(|image_data| {
                    let (width, height) = image_data.dimensions();
                    Vec2::new(width as f32, height as f32)
                });

        if let Some(image_size) = current_image_size {
            zoom_handler
                .update_for_window_resize(image_size, panel_rect.size());
        }

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

            let (image_rect, response) = Self::handle_image_positioning(
                ui,
                panel_rect,
                scaled_size,
                zoom_handler,
            );

            if response.dragged() {
                zoom_handler.add_offset(response.drag_delta());
            }

            let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                Self::handle_zoom(
                    ui,
                    zoom_handler,
                    scroll_delta.into(),
                    panel_rect,
                );
            }

            render_resolution_indicator(
                ui,
                image_manager.get_current_dimensions(),
                &config.indicator,
            );
            Self::render_zoom_indicator(ui, zoom_handler, &config.indicator);
            Self::render_filename_indicator(
                ui,
                image_manager.current_path.as_ref(),
                &config.indicator,
            );
            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }

    fn handle_zoom(
        ui: &Ui,
        zoom_handler: &mut ZoomHandler,
        scroll_delta: f64,
        panel_rect: Rect,
    ) {
        let zoom_step = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
        let new_zoom = (zoom_handler.zoom_level() * zoom_step).clamp(0.1, 10.0);
        let current_center = panel_rect.center() + zoom_handler.offset();

        if let Some(cursor_pos) = ui.input(|i| i.pointer.hover_pos()) {
            let cursor_to_center = cursor_pos - current_center;
            let scale_factor = new_zoom / zoom_handler.zoom_level();
            let new_cursor_to_center = cursor_to_center * scale_factor as f32;
            let offset_correction = cursor_to_center - new_cursor_to_center;
            zoom_handler.add_offset(offset_correction);
            zoom_handler.set_zoom(new_zoom);
        } else {
            zoom_handler.set_zoom(new_zoom);
        }

        ui.ctx().request_repaint();
    }

    fn handle_image_positioning(
        ui: &mut Ui,
        panel_rect: Rect,
        scaled_size: Vec2,
        zoom_handler: &ZoomHandler,
    ) -> (Rect, egui::Response) {
        let panel_center = panel_rect.center();
        let image_center = panel_center + zoom_handler.offset();
        let image_rect = Rect::from_center_size(image_center, scaled_size);
        let constrain_dragging = zoom_handler.get_fit_mode() != FitMode::Custom;
        let response = ui.allocate_rect(image_rect, Sense::drag());

        let final_rect = if constrain_dragging && response.dragged() {
            let drag_delta = response.drag_delta();
            let mut new_rect = image_rect.translate(drag_delta);
            let min_visible = 50.0;

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
        if !config.show_percentage {
            return;
        }

        let percentage_text = format!("{:.0}%", zoom_handler.zoom_percentage());
        let screen_rect = ui.ctx().screen_rect();
        let padding =
            Vec2::new(config.padding.x() as f32, config.padding.y() as f32);
        let font_size = config.font_size as f32;
        let char_width = font_size * 0.6;
        let text_width = char_width * percentage_text.len() as f32;
        let frame_margin = 8.0;
        let box_size = Vec2::new(
            text_width + frame_margin * 2.0,
            font_size + frame_margin * 2.0,
        );

        let pos = match config.position {
            Position::TopLeft => Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.min.y + padding.y,
            ),
            Position::TopRight => Pos2::new(
                screen_rect.max.x - box_size.x - padding.x,
                screen_rect.min.y + padding.y,
            ),
            Position::BottomLeft => Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.max.y - box_size.y - padding.y,
            ),
            Position::BottomRight => Pos2::new(
                screen_rect.max.x - box_size.x - padding.x,
                screen_rect.max.y - box_size.y - padding.y,
            ),
            Position::Top => Pos2::new(
                screen_rect.center().x - box_size.x / 2.0,
                screen_rect.min.y + padding.y,
            ),
            Position::Bottom => Pos2::new(
                screen_rect.center().x - box_size.x / 2.0,
                screen_rect.max.y - box_size.y - padding.y,
            ),
            Position::Left => Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.center().y - box_size.y / 2.0,
            ),
            Position::Right => Pos2::new(
                screen_rect.max.x - box_size.x - padding.x,
                screen_rect.center().y - box_size.y / 2.0,
            ),
            Position::Center => Pos2::new(
                screen_rect.center().x - box_size.x / 2.0,
                screen_rect.center().y - box_size.y / 2.0,
            ),
        };

        Area::new("zoom_indicator")
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
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

    fn render_filename_indicator(
        ui: &mut Ui,
        path: Option<&PathBuf>,
        config: &IndicatorConfig,
    ) {
        if let Some(path) = path {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            let screen_rect = ui.ctx().screen_rect();
            let padding =
                Vec2::new(config.padding.x() as f32, config.padding.y() as f32);
            let font_size = config.font_size as f32;
            let char_width = font_size * 0.6;
            let text_width = char_width * filename.len() as f32;
            let frame_margin = 8.0;
            let box_size = Vec2::new(
                text_width + frame_margin * 2.0,
                font_size + frame_margin * 2.0,
            );

            // Position in top left
            let Position_pos = Pos2::new(
                screen_rect.min.x + padding.x,
                screen_rect.min.y + padding.y,
            );

            Area::new("filename_indicator")
                .order(Order::Foreground)
                .fixed_pos(Position_pos)
                .show(ui.ctx(), |ui| {
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
                            let rich_text = RichText::new(filename)
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
}

fn render_resolution_indicator(
    ui: &mut Ui,
    dimensions: Option<(u32, u32)>,
    config: &IndicatorConfig,
) {
    if let Some((width, height)) = dimensions {
        let resolution_text = format!("{}x{}", width, height);
        let screen_rect = ui.ctx().screen_rect();
        let padding =
            Vec2::new(config.padding.x() as f32, config.padding.y() as f32);
        let font_size = config.font_size as f32;
        let char_width = font_size * 0.6;
        let text_width = char_width * resolution_text.len() as f32;
        let frame_margin = 8.0;
        let box_size = Vec2::new(
            text_width + frame_margin * 2.0,
            font_size + frame_margin * 2.0,
        );

        let pos = Pos2::new(
            screen_rect.center().x - box_size.x / 2.0,
            screen_rect.min.y + padding.y,
        );

        Area::new("resolution_indicator")
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
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
                        let rich_text = RichText::new(resolution_text)
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
