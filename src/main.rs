#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod models;
mod platform;
mod state;
mod theme;
mod views;
mod wallpaper;

use app::TimetableApp;
use eframe::NativeOptions;
use egui::vec2;

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(vec2(1140.0, 840.0))
            .with_min_inner_size(vec2(420.0, 600.0))
            .with_title("My Timetable · UniMAP")
            .with_resizable(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "UniMAP Timetable",
        native_options,
        Box::new(|cc| Ok(Box::new(TimetableApp::new(cc)))),
    )
}
