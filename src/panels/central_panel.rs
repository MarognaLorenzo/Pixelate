use std::ops::Add;
use easy_paths::get_absolute_path;
use egui::Context;
use image::GenericImageView;
use image::imageops::Gaussian;
use crate::GUI;

impl GUI {
    pub fn load_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.layout().vertical_align().to_factor();
            ui.layout().horizontal_align().to_factor();
            ui.heading("Hello World!");
            if let Some(active_image) = &self.active_image {
                let th = &ui.ctx().load_texture(
                    "My picture",
                    egui::ColorImage::from_rgb([active_image.0.dimensions().0 as usize, active_image.0.dimensions().1 as usize],
                                               active_image.0.as_bytes()),
                    Default::default(),
                );
                ui.image(th, th.size_vec2());
            } else {}
        });
    }
}

