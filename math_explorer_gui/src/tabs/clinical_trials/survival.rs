use crate::tabs::clinical_trials::ClinicalTrialTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::applied::clinical_trials::survival_analysis::{kaplan_meier, Observation};
use math_explorer::applied::clinical_trials::types::SurvivalTime;

pub struct SurvivalAnalysisTool {
    input_text: String,
    result_plot: Option<Vec<[f64; 2]>>,
    error_msg: Option<String>,
}

impl Default for SurvivalAnalysisTool {
    fn default() -> Self {
        Self {
            input_text: "10 1\n20 0\n30 1\n40 1\n50 0".to_string(),
            result_plot: None,
            error_msg: None,
        }
    }
}

impl ClinicalTrialTool for SurvivalAnalysisTool {
    fn name(&self) -> &'static str {
        "Survival Analysis (Kaplan-Meier)"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Enter survival data (Time Event). Event: 1=Event, 0=Censored.");

        ui.add(
            egui::TextEdit::multiline(&mut self.input_text)
                .desired_width(f32::INFINITY)
                .desired_rows(5),
        );

        if ui.button("Calculate Kaplan-Meier").clicked() {
            self.calculate();
        }

        if let Some(err) = &self.error_msg {
            ui.colored_label(egui::Color32::RED, err);
        }

        if let Some(points) = &self.result_plot {
            let plot_points = PlotPoints::new(points.clone());
            let line = Line::new("Survival Probability", plot_points);

            Plot::new("km_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        }
    }
}

impl SurvivalAnalysisTool {
    fn calculate(&mut self) {
        self.error_msg = None;
        let mut observations = Vec::new();

        for (line_idx, line) in self.input_text.lines().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            if parts.len() != 2 {
                self.error_msg = Some(format!("Line {}: Expected 2 values (Time Event)", line_idx + 1));
                return;
            }

            let time_val: f64 = match parts[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    self.error_msg = Some(format!("Line {}: Invalid time value", line_idx + 1));
                    return;
                }
            };

            let event_val: u8 = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    self.error_msg = Some(format!("Line {}: Invalid event value (0 or 1)", line_idx + 1));
                    return;
                }
            };

            let event_occurred = match event_val {
                1 => true,
                0 => false,
                _ => {
                    self.error_msg = Some(format!("Line {}: Event must be 0 or 1", line_idx + 1));
                    return;
                }
            };

            let time = match SurvivalTime::new(time_val) {
                Ok(t) => t,
                Err(_) => {
                     self.error_msg = Some(format!("Line {}: Time must be non-negative", line_idx + 1));
                     return;
                }
            };

            observations.push(Observation {
                time,
                event_occurred,
            });
        }

        if observations.is_empty() {
            self.error_msg = Some("No valid data provided.".to_string());
            return;
        }

        let curve = kaplan_meier(&observations);

        let mut step_points = Vec::new();
        let mut prev_time = 0.0;
        let mut prev_survival = 1.0;

        // Start point
        step_points.push([0.0, 1.0]);

        for point in curve {
            // Horizontal to next time
            step_points.push([point.time, prev_survival]);
            // Vertical down
            step_points.push([point.time, point.survival_probability]);

            prev_time = point.time;
            prev_survival = point.survival_probability;
        }
        // Extend slightly
        step_points.push([prev_time * 1.1, prev_survival]);

        self.result_plot = Some(step_points);
    }
}
