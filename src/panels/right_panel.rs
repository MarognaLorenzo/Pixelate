use egui::{Context, Id};
use image::GenericImageView;
use crate::MyEguiApp;

impl MyEguiApp {
    pub fn right_panel(&mut self, ctx: &Context) {
        egui::SidePanel::right(Id::new("Right panel")).show(ctx, |ui| {
            ui.push_id("Combo_box_for_image_selection", |ui| {
                egui::ComboBox::from_label("Combobox").selected_text(self.selected_number.to_string())

                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_number, 1, "1");
                        ui.selectable_value(&mut self.selected_number, 2, "2");
                    })

            },
            )
        });
    }
}

