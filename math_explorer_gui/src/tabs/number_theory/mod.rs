use crate::tabs::ExplorerTab;
use eframe::egui;

mod prime_spiral;
use prime_spiral::PrimeSpiralWidget;

#[derive(Default)]
pub struct NumberTheoryTab {
    prime_spiral: PrimeSpiralWidget,
}

impl ExplorerTab for NumberTheoryTab {
    fn name(&self) -> &'static str {
        "Number Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Number Theory");
            ui.label("Explore properties of integers.");
            ui.separator();

            // Navigation for sub-tools could go here.
            // For now, we only have Prime Spiral.

            ui.heading("Prime Spiral (Ulam Spiral)");
            ui.label("A graphical depiction of the set of prime numbers, revealed by writing the positive integers in a square spiral and marking the prime numbers.");

            ui.add_space(10.0);
            self.prime_spiral.ui(ui);
        });
    }
}
