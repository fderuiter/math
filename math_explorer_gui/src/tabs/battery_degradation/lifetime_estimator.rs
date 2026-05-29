use super::BatteryDegradationTool;
use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use applied::battery_degradation::{Capacity, DepthOfDischarge, PowerLawModel};

pub struct LifetimeEstimatorTool {
    target_capacity: f64,
    dod: f64,
    cycles_per_day: f64,
}

impl Default for LifetimeEstimatorTool {
    fn default() -> Self {
        Self {
            target_capacity: 80.0,
            dod: 50.0,
            cycles_per_day: 1.0,
        }
    }
}

impl BatteryDegradationTool for LifetimeEstimatorTool {
    fn name(&self) -> &'static str {
        "Lifetime Estimator"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        // Enforce valid ranges
        self.target_capacity = self.target_capacity.clamp(0.1, 99.9);
        self.dod = self.dod.clamp(0.1, 100.0);
        self.cycles_per_day = self.cycles_per_day.max(0.1);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Battery Lifetime Estimator");
            ui.label("Calculate the expected battery life based on specific usage profiles.");
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("Usage Profile");
                egui::Grid::new("lifetime_params_grid").show(ui, |ui| {
                    ui.label("Depth of Discharge (DoD):");
                    ui.add(egui::Slider::new(&mut self.dod, 0.1..=100.0).text("%"))
                        .accessible_hover_text("Percentage of battery capacity used per cycle.");
                    ui.end_row();

                    ui.label("Target Capacity (End of Life):");
                    ui.add(egui::Slider::new(&mut self.target_capacity, 10.0..=99.0).text("%"))
                        .accessible_hover_text(
                            "The threshold capacity to be considered 'end of life'.",
                        );
                    ui.end_row();

                    ui.label("Cycles per Day:");
                    ui.add(egui::Slider::new(&mut self.cycles_per_day, 0.1..=10.0).text("cycles"))
                        .accessible_hover_text("How many charge/discharge cycles happen per day.");
                    ui.end_row();
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Calculation
            let model = PowerLawModel::standard();
            let dod_val = DepthOfDischarge::new(self.dod);
            let capacity_val = Capacity::new(self.target_capacity / 100.0);

            let total_cycles = if let (Ok(c), Ok(d)) = (capacity_val, dod_val) {
                model.cycles_to_capacity(c, d).as_f64()
            } else {
                0.0
            };
            let lifetime_days = total_cycles / self.cycles_per_day;
            let lifetime_years = lifetime_days / 365.25;

            // Results Display
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Estimated Total Cycles:")
                        .size(18.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format!("{:.0}", total_cycles))
                        .size(24.0)
                        .color(egui::Color32::GREEN)
                        .strong(),
                );
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Estimated Lifetime:")
                        .size(18.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format!("{:.1} years", lifetime_years))
                        .size(24.0)
                        .color(egui::Color32::GREEN)
                        .strong(),
                );
            });

            ui.label(format!("({:.0} days)", lifetime_days));
        });
    }
}

// [cite:algorithmic_information_rust]
