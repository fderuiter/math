use crate::tabs::clinical_trials::ClinicalTrialTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::sample_size::calculate_sample_size_means;

pub struct SampleSizeTool {
    alpha: f64,
    power: f64,
    delta: f64,
    sigma: f64,
    result_n: Option<usize>,
    error_msg: Option<String>,
}

impl Default for SampleSizeTool {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            power: 0.80,
            delta: 5.0,
            sigma: 10.0,
            result_n: None,
            error_msg: None,
        }
    }
}

impl ClinicalTrialTool for SampleSizeTool {
    fn name(&self) -> &'static str {
        "Sample Size Calculator (Means)"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Parameters");

        ui.add(egui::Slider::new(&mut self.alpha, 0.001..=0.20).text("Alpha (Type I Error)"));
        ui.add(egui::Slider::new(&mut self.power, 0.50..=0.99).text("Power (1 - Beta)"));

        ui.add(egui::Slider::new(&mut self.delta, 0.1..=100.0).text("Effect Size (Delta)"));
        ui.add(egui::Slider::new(&mut self.sigma, 0.1..=100.0).text("Standard Deviation (Sigma)"));

        if ui.button("Calculate Sample Size").clicked() {
            match calculate_sample_size_means(self.alpha, self.power, self.delta, self.sigma) {
                Ok(n) => {
                    self.result_n = Some(n);
                    self.error_msg = None;
                }
                Err(e) => {
                    self.error_msg = Some(e.to_string());
                    self.result_n = None;
                }
            }
        }

        ui.separator();

        if let Some(err) = &self.error_msg {
            ui.colored_label(egui::Color32::RED, err);
        }

        if let Some(n) = self.result_n {
            ui.heading(format!("Required Sample Size per Group: {}", n));
            ui.label(format!("Total Sample Size (2 groups): {}", n * 2));
        }
    }
}
