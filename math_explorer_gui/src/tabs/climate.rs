use crate::tabs::ExplorerTab;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct ClimateTab {
    // We will generate some dummy data for the global temperature anomalies.
    // In a real application, this would come from a dataset.
    time_series: Vec<[f64; 2]>,
}

impl Default for ClimateTab {
    fn default() -> Self {
        Self {
            time_series: math_explorer::climate::dataset::get_temperature_anomalies(),
        }
    }
}

impl ExplorerTab for ClimateTab {
    fn name(&self) -> &'static str {
        "Climate"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Climate Modeling");
            ui.separator();

            ui.label("Global Temperature Anomalies:");

            let plot = Plot::new("temperature_anomalies_plot")
                .view_aspect(2.0)
                .x_axis_formatter(|x, _range| format!("{:.0}", x.value))
                .y_axis_formatter(|y, _range| format!("{:.2}°C", y.value))
                .legend(egui_plot::Legend::default());

            plot.show(ui, |plot_ui| {
                let points = PlotPoints::new(self.time_series.clone());
                let line = Line::new("Temperature Anomaly", points)
                    .width(2.0);
                plot_ui.line(line);
            });
        });
    }
}
