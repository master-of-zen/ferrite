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

        let screen_rect = ui.ctx().screen_rect();
        egui::Area::new("help_menu")
            .fixed_pos(egui::pos2(
                screen_rect.center().x - 120.0,
                screen_rect.center().y - 80.0,
            ))
            .show(ui.ctx(), |ui| {
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(
                        config.background_color.r,
                        config.background_color.g,
                        config.background_color.b,
                        config.background_color.a,
                    ))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.set_max_width(240.0);
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                egui::RichText::new("Navigation")
                                    .color(Color32::from_rgba_unmultiplied(
                                        config.text_color.r,
                                        config.text_color.g,
                                        config.text_color.b,
                                        config.text_color.a,
                                    ))
                                    .size(18.0),
                            );
                            ui.add_space(8.0);
                            let text_color = Color32::from_rgba_unmultiplied(
                                config.text_color.r,
                                config.text_color.g,
                                config.text_color.b,
                                config.text_color.a,
                            );
                            ui.label(
                                egui::RichText::new("← or A: Previous image")
                                    .color(text_color),
                            );
                            ui.label(
                                egui::RichText::new("→ or D: Next image")
                                    .color(text_color),
                            );
                            ui.label(
                                egui::RichText::new("Q: Quit")
                                    .color(text_color),
                            );
                        });
                    });
            });
    }
}
