use egui::Id;
use crate::GUI;

impl GUI {
    pub fn load_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left(Id::new(3)).show(ctx, |ui| {
            ui.heading("left panel");
        });
    }
}