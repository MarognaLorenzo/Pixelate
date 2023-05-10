use egui::Context;
use image::GenericImageView;
use crate::GUI;

impl GUI {
    pub fn load_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.layout().vertical_align().to_factor();
            ui.layout().horizontal_align().to_factor();
            ui.heading("Hello World!");
            if let Some(updating_image) = &self.cached_image {
                let th = &ui.ctx().load_texture(
                    "My picture",
                    {
                        let mut image = updating_image.0.clone();
                        if self.enable_filter.brighten {
                            let amount = updating_image.1.brighten - self.effects.brighten;
                            let tmp = image.brighten(amount);
                            image = tmp;
                        }
                        if self.enable_filter.blur {
                            let amount = updating_image.1.blur - self.effects.blur;
                            let tmp = image.blur(amount);
                            image = tmp;
                        }
                        if self.enable_filter.contrast {
                            let amount = updating_image.1.contrast - self.effects.contrast;
                            let tmp = image.adjust_contrast(amount);
                            image = tmp;
                        }
                        egui::ColorImage::from_rgb([image.dimensions().0 as usize, image.dimensions().1 as usize],image.as_bytes())
                    },
                    Default::default(),
                );
                ui.image(th, th.size_vec2());
            } else {}
        });
    }
}

