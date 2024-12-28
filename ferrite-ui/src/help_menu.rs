use eframe::egui;
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
        let window = egui::Window::new("Help")
            .collapsible(false)
            .resizable(false)
            .fixed_pos(egui::pos2(
                screen_rect.center().x - 150.0,
                screen_rect.center().y - 100.0,
            ))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Navigation");
                    ui.add_space(8.0);
                    ui.label("← or A: Previous image");
                    ui.label("→ or D: Next image");
                    ui.label("Q: Quit");
                });
            });
    }
}
