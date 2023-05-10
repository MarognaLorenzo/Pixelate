use egui::{Id};
use image::{DynamicImage, GenericImage};
use crate::{Effects, GUI};


impl GUI {
    pub fn load_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom(Id::new(1)).show(
            ctx,
            |ui| {
                ui.heading("bottom altro");

                if let Some(image) = &mut self.cached_image && let Some(base) = &self.base_image {
                    let brighten_slider = ui.add(egui::Slider::new(&mut image.1.brighten, -100..=100)
                        .text("Brighten"));

                    if brighten_slider.drag_started() {
                        self.enable_filter.brighten = true;
                    }
                    if brighten_slider.drag_released() {
                        self.effects.brighten = image.1.brighten;
                        image.0.copy_from(
                            &base.0
                                .brighten(self.effects.brighten)
                                .adjust_contrast(self.effects.contrast)
                                .blur(self.effects.blur),
                            0,
                            0,
                        ).expect("copying failed");
                        self.enable_filter.brighten = false;
                    }

                    let contrast_slider = ui.add(egui::Slider::new(&mut image.1.contrast, -40.0..=140.0).text("Contrast"));
                    if contrast_slider.drag_started() {
                        self.enable_filter.contrast = true;
                    }
                    if contrast_slider.drag_released() {
                        self.effects.contrast = image.1.contrast;
                        image.0.copy_from(
                            &base.0
                                .brighten(self.effects.brighten)
                                .adjust_contrast(self.effects.contrast)
                                .blur(self.effects.blur),
                            0,
                            0,
                        ).expect("copying failed");
                        self.enable_filter.contrast = false;
                    }


                    // let blur_slider = ui.add(egui::Slider::new(&mut image.1.blur, -100.0 ..=100.0).text("Blur"));
                    // if blur_slider.drag_started() {
                    //     self.enable_filter.blur = true;
                    // }
                    // if blur_slider.drag_released() {
                    //     self.effects.blur = image.1.blur;
                    //     image.0.copy_from(
                    //         &base.0
                    //             .brighten(self.effects.brighten)
                    //             .adjust_contrast(self.effects.contrast)
                    //             .blur(self.effects.blur),
                    //         0,
                    //         0,
                    //     ).expect("copying failed");
                    //     self.enable_filter.blur = false;
                    // }
                    ui.add(egui::Checkbox::new(&mut self.effects.grayscale, "Grayscale"));
                }
            }
        );
    }
}
