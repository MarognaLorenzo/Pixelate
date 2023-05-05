mod functions;
pub mod panels;

use eframe::egui;
use image::{ColorType, DynamicImage, GenericImageView};
use easy_paths::get_absolute_path;
use egui::{Context, FontDefinitions, Id, Response};
use egui::panel::TopBottomSide;
use egui::WidgetType::ComboBox;
use image::imageops::{FilterType, Gaussian};


use crate::functions::{avarage, manupulation, separate_colors, sum_images};

fn main() {
    manupulation();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    active_image: DynamicImage,
    selected_number: i32
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self{
            active_image: {
                let path = get_absolute_path(&"./images/samples/retina.jpg");
                println!("{:?}", path);
                let img = image::open(path).unwrap().resize(500,200, Gaussian);
                img
            },
            selected_number: 1,
        }

    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        println!("{:?}", get_absolute_path(&"./images/samples/retina.jpg"));
        self.central_panel(ctx);
        self.top_panel(ctx);
        self.right_panel(ctx);
        self.left_panel(ctx);
        self.bottom_panel(ctx);
    }
}