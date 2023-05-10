use std::ops::Add;
use easy_paths::get_absolute_path;
use egui::{Context, Id};
use image::GenericImageView;
use image::imageops::Gaussian;
use crate::GUI;

impl GUI {
    pub fn load_right_panel(&mut self, ctx: &Context) {
        egui::SidePanel::right(Id::new("Right panel")).show(ctx, |ui| {
            if self.active_image.is_none() {
                ui.push_id("Combo_box_for_image_selection", |ui| {
                    egui::ComboBox::from_label("Combobox").selected_text(self.selected_image.to_string())
                        .show_ui(ui, |ui| {
                            for image in &self.available_images {
                                ui.selectable_value(&mut self.selected_image, image.clone(), image);
                            }
                        })
                },
                );
            } else {
                if ui.button("Remove image").clicked() {
                    self.active_image = None;
                    self.selected_image = String::default();
                }
            }
        });
    }
}

