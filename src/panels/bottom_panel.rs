use egui::Id;
use crate::MyEguiApp;

impl MyEguiApp {
    pub fn bottom_panel(&mut self, ctx: & egui::Context) {
        egui::TopBottomPanel::bottom(Id::new(1)).show(ctx, |ui| {
            ui.heading("bottom altro");
        });
    }
}