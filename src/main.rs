mod app;
mod terminal;
mod pty;
mod theme;
mod completer;
mod commands;

use app::IgnisApp;
use eframe::{egui, Renderer};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Ignis Terminal"),
        renderer: Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "Ignis Terminal",
        options,
        Box::new(|cc| Box::new(IgnisApp::new(cc))),
    )
}