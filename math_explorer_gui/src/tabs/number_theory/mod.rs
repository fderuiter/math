use crate::tabs::ExplorerTab;
use eframe::egui;

mod prime_spiral;
mod ualbf_widget;
use prime_spiral::PrimeSpiralWidget;
use ualbf_widget::UalbfWidget;

#[derive(Default)]
pub struct NumberTheoryTab {
    prime_spiral: PrimeSpiralWidget,
    ualbf: UalbfWidget,
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
            ui.heading("Prime Spiral (Ulam Spiral)");
            ui.label("A graphical depiction of the set of prime numbers...");
            ui.add_space(10.0);
            self.prime_spiral.ui(ui);
            ui.separator();
            ui.heading("UALBF: Unified Algebraic-Lattice Bipartition Framework");
            ui.label(
                "A synthesis of ALCF and AMBS for proving lower bounds on quasiperfect numbers.",
            );
            ui.add_space(10.0);
            self.ualbf.ui(ui);
        });
    }
}
