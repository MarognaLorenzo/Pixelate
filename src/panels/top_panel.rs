use std::ops::Add;
use easy_paths::{get_absolute_path, is_file};
use egui::{Context, Id};
use image::imageops::Gaussian;
use crate::GUI;
use easy_paths::is_existing_path;

impl GUI {
    pub fn load_top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top(Id::new(0)).show(ctx, |ui| {
            ui.heading("top qualcosa");
            if self.active_image.is_none() {
                if ui.button("Upload image").clicked() {
                    let path = "./images/samples/".to_string().add(self.selected_image.as_str());
                    let abs_path = get_absolute_path(&path);
                    if is_file(&abs_path){
                        let img = image::open(abs_path).unwrap();
                        self.active_image = Some((img, self.selected_image.to_string()));
                    }
                }
            }
        });
    }
}