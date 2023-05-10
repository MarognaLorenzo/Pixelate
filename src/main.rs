mod functions;
pub mod panels;

use eframe::egui;
use image::{ColorType, DynamicImage, GenericImageView};
use easy_paths::get_absolute_path;
use egui::{Context, FontDefinitions, Id, Response};
use egui::panel::TopBottomSide;
use egui::WidgetType::ComboBox;
use image::imageops::{FilterType, Gaussian};
use walkdir::WalkDir;


use crate::functions::{avarage, manupulation, separate_colors, sum_images};

fn main() {
    manupulation();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(GUI::new(cc))));
}

#[derive(Default)]
struct GUI {
    active_image: Option<(DynamicImage, String)>,
    selected_image: String,
    available_images: Vec<String>
}

impl GUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self{
            active_image: None,
            selected_image: String::default(),
            available_images: {
                let mut res = vec![];
                for file in WalkDir::new("./images/samples").into_iter().filter_map(|file| file.ok()) {
                    let path = file.path().strip_prefix("./images/samples").unwrap().display();
                    if path.to_string() != String::default() {
                        res.push(path.to_string());
                    }
                }
                res
            }
        }

    }

}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_title("Pixelate");
        frame.set_maximized(true);
        self.load_central_panel(ctx);
        self.load_top_panel(ctx);
        self.load_right_panel(ctx);
        // self.load_left_panel(ctx);
        self.load_bottom_panel(ctx);
    }
}