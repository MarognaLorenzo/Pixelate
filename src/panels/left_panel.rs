use egui::Id;
use crate::MyEguiApp;

impl MyEguiApp {
    pub fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left(Id::new(3)).show(ctx, |ui| {
            ui.heading("left panel");
        });
    }
}