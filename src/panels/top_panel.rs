use egui::{Context, Id};
use crate::MyEguiApp;

impl MyEguiApp {
    pub fn top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top(Id::new(0)).show(ctx, |ui| {
            ui.heading("top qualcosa");
        });
    }
}