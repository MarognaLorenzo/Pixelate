#![feature(let_chains)]

mod functions;
pub mod panels;

use std::default::Default;
use eframe::egui;
use image::{DynamicImage};
use egui::{Id, Response};
use walkdir::WalkDir;

#[derive(Default, Clone)]
pub struct Effects {
    brighten: i32,
    blur: f32,
    contrast: f32,
    grayscale: bool,
}

impl Effects {
    pub fn new() -> Effects {
        Effects {
            brighten: 0,
            blur: 0.0,
            contrast: 0.0,
            grayscale: false,
        }
    }
}

#[derive(Default, Clone)]
pub struct Filters_to_apply {
    brighten: bool,
    blur: bool,
    contrast: bool,
}

impl Filters_to_apply {
    pub fn new() -> Filters_to_apply {
    Filters_to_apply{
        brighten: false,
        blur: false,
        contrast: false,
    }
    }
}

use crate::functions::{avarage, manupulation, separate_colors, sum_images};

fn main() {
    manupulation();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(GUI::new(cc)))).expect("panic message");
}

#[derive(Default)]
struct GUI {
    base_image: Option<(DynamicImage, String)>,
    cached_image: Option<(DynamicImage, Effects)>,
    selected_image: String,
    available_images: Vec<String>,
    effects: Effects,
    export_name: String,
    enable_filter: Filters_to_apply,
}

impl GUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self{
            base_image: None,
            cached_image: None,
            selected_image: String::default(),
            available_images: get_available_images(),
            effects: Effects::new(),
            export_name: String::default(),
            enable_filter: Filters_to_apply::new(),
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

fn get_available_images() -> Vec<String> {
    let mut res = vec![];
    for file in WalkDir::new("./images/samples").into_iter().filter_map(|file| file.ok()) {
        let path = file.path().strip_prefix("./images/samples").unwrap().display();
        if path.to_string() != String::default() {
            res.push(path.to_string());
        }
    }
    res
}