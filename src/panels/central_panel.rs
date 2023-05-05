use egui::Context;
use image::GenericImageView;
use crate::MyEguiApp;

impl MyEguiApp {
    pub fn central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.layout().cross_align();
            ui.heading("Hello World!");
            let _ = ui.button("where are you?");
            let th = &ui.ctx().load_texture(
                "My picture",
                egui::ColorImage::from_rgb([self.active_image.dimensions().0 as usize, self.active_image.dimensions().1 as usize],
                                           self.active_image.as_bytes()),
                Default::default(),
            );
            ui.image(th, th.size_vec2())
        });
    }
}

