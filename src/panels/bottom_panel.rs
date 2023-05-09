use egui::Id;
use crate::GUI;

impl GUI {
    pub fn load_bottom_panel(&mut self, ctx: & egui::Context) {
        egui::TopBottomPanel::bottom(Id::new(1)).show(ctx, |ui| {
            ui.heading("bottom altro");
        });
    }
}