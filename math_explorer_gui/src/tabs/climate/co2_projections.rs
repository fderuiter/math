use crate::accessibility::PlotAccessibilityExt;
use crate::tabs::climate::ClimateTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct Co2ProjectionsTool {
    reduction_scenario: f64,
}

impl Default for Co2ProjectionsTool {
    fn default() -> Self {
        Self {
            reduction_scenario: 0.0,
        }
    }
}

impl ClimateTool for Co2ProjectionsTool {
    fn name(&self) -> &'static str {
        "CO2 Projections"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Global CO2 Concentration Projections:");
        ui.add(
            egui::Slider::new(&mut self.reduction_scenario, 0.0..=1.0)
                .text("Emission Reduction Scenario"),
        );

        let (historical, projected) =
            math_explorer::climate::dataset::get_co2_projections(self.reduction_scenario);

        let plot = Plot::new("co2_projections_plot")
            .view_aspect(2.0)
            .x_axis_formatter(|x, _range| format!("{:.0}", x.value))
            .y_axis_formatter(|y, _range| format!("{:.1} ppm", y.value))
            .legend(egui_plot::Legend::default());

        plot.show_accessible(
            ui,
            "Dynamic state of co2_projections_plot updated.",
            |plot_ui| {
                let hist_points = PlotPoints::new(historical);
                let hist_line = Line::new("Historical CO2", hist_points).width(2.0_f32);
                plot_ui.line(hist_line);

                let proj_points = PlotPoints::new(projected);
                let proj_line = Line::new("Projected CO2", proj_points)
                    .width(2.0_f32)
                    .color(egui::Color32::RED);
                plot_ui.line(proj_line);
            },
        );
    }
}

// [cite:isosurface_extraction]
