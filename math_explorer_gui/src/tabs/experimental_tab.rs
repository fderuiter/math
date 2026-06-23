use crate::tabs::ExplorerTab;
use eframe::egui;

#[allow(unused_imports)]
use generative_turbulence_experimental;

#[derive(Default)]
pub struct ExperimentalTab {}

impl ExplorerTab for ExperimentalTab {
    fn name(&self) -> &'static str {
        "Generative Turbulence (Experimental)"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Generative Turbulence");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("⚠️ Unstable / Experimental Feature")
                        .color(egui::Color32::RED)
                        .strong(),
                );
            });
            ui.add_space(20.0);

            ui.label(
                "This feature is currently experimental and may produce unstable results or crash.",
            );
            ui.label("Generative Turbulence module is loaded from the experimental subspace.");
        });
    }
}
// [cite:graph_parameters_rust]
