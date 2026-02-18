mod app;

use app::MriApp;
use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Math Explorer: MRI Bloch Simulator"),
        ..Default::default()
    };

    eframe::run_native(
        "Math Explorer: MRI Bloch Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(MriApp::new(cc)))),
    )
}
