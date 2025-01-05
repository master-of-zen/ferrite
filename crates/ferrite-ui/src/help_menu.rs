use eframe::egui::{self, Color32, Frame, Rounding};
use ferrite_config::HelpMenuConfig;

pub struct HelpMenu {
    visible: bool,
}

impl HelpMenu {
    pub fn new() -> Self {
        Self {
            visible: false
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn render(&self, ui: &mut egui::Ui, config: &HelpMenuConfig) {
        if !self.visible {
            return;
        }

        let base_font_size = config.font_size as f32;
        let heading_size = base_font_size * 1.2;
        let row_height = base_font_size * 1.5;
        let spacing = row_height * 0.5;
        let column_width = base_font_size * 10.0;
        let total_width = column_width * 3.0 + spacing * 2.0;

        let screen_rect = ui.ctx().screen_rect();
        egui::Area::new("help_menu".into())
            .fixed_pos(egui::pos2(
                screen_rect.center().x - total_width * 0.5,
                screen_rect.center().y - (heading_size + row_height * 4.0),
            ))
            .show(ui.ctx(), |ui| {
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(
                        config.background_color.r,
                        config.background_color.g,
                        config.background_color.b,
                        config.background_color.a,
                    ))
                    .rounding(Rounding::same(row_height * 0.5))
                    .inner_margin(spacing)
                    .show(ui, |ui| {
                        ui.set_max_width(total_width);

                        ui.columns(3, |columns| {
                            for col in columns.iter_mut() {
                                col.set_max_width(column_width);
                            }

                            columns[0].vertical(|ui| {
                                render_section(
                                    ui,
                                    "Navigation",
                                    &[
                                        "LEFT or A: Previous",
                                        "RIGHT or D: Next",
                                    ],
                                    config,
                                    heading_size,
                                )
                            });

                            columns[1].vertical(|ui| {
                                render_section(
                                    ui,
                                    "Zoom",
                                    &[
                                        "Mouse Wheel",
                                        "+ or W: Zoom in",
                                        "- or S: Zoom out",
                                        "0: Reset zoom",
                                        "F: Toggle fit",
                                    ],
                                    config,
                                    heading_size,
                                )
                            });

                            columns[2].vertical(|ui| {
                                render_section(
                                    ui,
                                    "Other",
                                    &["H: Toggle help", "Q: Quit"],
                                    config,
                                    heading_size,
                                )
                            });
                        });
                    });
            });
    }
}

#[inline]
fn render_section(
    ui: &mut egui::Ui,
    title: &str,
    items: &[&str],
    config: &HelpMenuConfig,
    heading_size: f32,
) {
    ui.heading(
        egui::RichText::new(title)
            .color(Color32::from_rgba_unmultiplied(
                config.text_color.r,
                config.text_color.g,
                config.text_color.b,
                config.text_color.a,
            ))
            .size(heading_size),
    );

    ui.add_space(config.font_size as f32 * 0.5);

    let text_color = Color32::from_rgba_unmultiplied(
        config.text_color.r,
        config.text_color.g,
        config.text_color.b,
        config.text_color.a,
    );

    for item in items {
        ui.label(
            egui::RichText::new(*item)
                .color(text_color)
                .size(config.font_size as f32),
        );
    }
}
