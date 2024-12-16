use eframe::egui::{self, Context, Ui};
use ferrite_config::FeriteConfig;

pub struct MenuBar {
    hidden: bool,
}

impl MenuBar {
    pub fn new(hidden: bool) -> Self {
        Self {
            hidden,
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn toggle(&mut self) {
        self.hidden = !self.hidden;
    }

    pub fn render(
        &self,
        ui: &mut Ui,
        ctx: &Context,
        config: &mut FeriteConfig,
    ) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Toggle Performance").clicked() {
                    config.show_performance = !config.show_performance;
                    ui.close_menu();
                }
                if ui.button("Toggle Menu (M)").clicked() {
                    config.window.hide_menu = !config.window.hide_menu;
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("Zoom In (+)").clicked() {
                    ctx.request_repaint();
                    ui.close_menu();
                }
                if ui.button("Zoom Out (-)").clicked() {
                    ctx.request_repaint();
                    ui.close_menu();
                }
                if ui.button("Reset Zoom (0)").clicked() {
                    ctx.request_repaint();
                    ui.close_menu();
                }
            });
        });
    }
}
