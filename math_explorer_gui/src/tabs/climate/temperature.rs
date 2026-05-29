use crate::tabs::climate::ClimateTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct TemperatureAnomaliesTool {
    time_series: Vec<[f64; 2]>,
}

impl Default for TemperatureAnomaliesTool {
    fn default() -> Self {
        Self {
            time_series: climate::dataset::get_temperature_anomalies(),
        }
    }
}

impl ClimateTool for TemperatureAnomaliesTool {
    fn name(&self) -> &'static str {
        "Temperature Anomalies"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Global Temperature Anomalies:");

        let plot = Plot::new("temperature_anomalies_plot")
            .view_aspect(2.0)
            .x_axis_formatter(|x, _range| format!("{:.0}", x.value))
            .y_axis_formatter(|y, _range| format!("{:.2}°C", y.value))
            .legend(egui_plot::Legend::default());

        plot.show(ui, |plot_ui| {
            let points = PlotPoints::new(self.time_series.clone());
            let line = Line::new("Temperature Anomaly", points).width(2.0_f32);
            plot_ui.line(line);
        });
    }
}

// [cite:dwarf_galaxy_empirical_dependencies]
