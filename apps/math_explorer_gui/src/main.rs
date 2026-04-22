mod app;
mod tabs;

use app::MathExplorerApp;
use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Math Explorer"),
        ..Default::default()
    };

    eframe::run_native(
        "Math Explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(MathExplorerApp::new(cc)))),
    )
}
