use std::ops::Add;
use egui::{Context, Id};

use crate::{Effects, GUI};

impl GUI {
    pub fn load_right_panel(&mut self, ctx: &Context) {
        egui::SidePanel::right(Id::new("Right panel")).show(ctx, |ui| {
            if self.cached_image.is_none() {
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
                    self.cached_image = None;
                    self.selected_image = String::default();
                }
                let e_button = ui.button("Export image");
                ui.text_edit_singleline(&mut self.export_name);
                if e_button.clicked() {
                    self.cached_image.as_ref().unwrap().0.save("images/output/".to_string().add(&*self.export_name).add(".jpg")).expect("yeahh");
                    self.cached_image = None;
                    self.selected_image = String::default();
                    self.effects = Effects::new();
                }
            }
        });
    }
}

