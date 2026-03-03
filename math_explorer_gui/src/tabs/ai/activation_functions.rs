use super::AiTool;
use eframe::egui;
use egui_plot::{Line, LineStyle, Plot, PlotPoints};
use math_explorer::ai::deep_learning_theory::calculus::{
    gelu, gelu_prime, relu, relu_prime, sigmoid, sigmoid_prime, tanh, tanh_prime,
};
use nalgebra::DVector;

#[derive(PartialEq, Clone, Copy)]
enum ActivationFunction {
    ReLU,
    Sigmoid,
    Tanh,
    GELU,
}

impl ActivationFunction {
    fn name(&self) -> &'static str {
        match self {
            ActivationFunction::ReLU => "ReLU",
            ActivationFunction::Sigmoid => "Sigmoid",
            ActivationFunction::Tanh => "Tanh",
            ActivationFunction::GELU => "GELU",
        }
    }
}

pub struct ActivationFunctionsTool {
    selected_function: ActivationFunction,
    show_derivative: bool,
    x_min: f64,
    x_max: f64,
    points_count: usize,
}

impl Default for ActivationFunctionsTool {
    fn default() -> Self {
        Self {
            selected_function: ActivationFunction::ReLU,
            show_derivative: false,
            x_min: -5.0,
            x_max: 5.0,
            points_count: 500,
        }
    }
}

impl AiTool for ActivationFunctionsTool {
    fn name(&self) -> &'static str {
        "Activation Functions"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("activation_controls").show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.label("Select Function:");
            ui.radio_value(
                &mut self.selected_function,
                ActivationFunction::ReLU,
                "ReLU",
            );
            ui.radio_value(
                &mut self.selected_function,
                ActivationFunction::Sigmoid,
                "Sigmoid",
            );
            ui.radio_value(
                &mut self.selected_function,
                ActivationFunction::Tanh,
                "Tanh",
            );
            ui.radio_value(
                &mut self.selected_function,
                ActivationFunction::GELU,
                "GELU",
            );

            ui.separator();
            ui.checkbox(&mut self.show_derivative, "Show Derivative");

            ui.separator();
            ui.label("Domain:");
            ui.add(egui::Slider::new(&mut self.x_min, -10.0..=-1.0).text("Min X"));
            ui.add(egui::Slider::new(&mut self.x_max, 1.0..=10.0).text("Max X"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Activation Function Plot");

            // Generate data points
            let step = (self.x_max - self.x_min) / (self.points_count as f64 - 1.0);
            let x_vals: Vec<f64> = (0..self.points_count)
                .map(|i| self.x_min + i as f64 * step)
                .collect();
            let x_vec = DVector::from_vec(x_vals.clone());

            let (y_vals, y_prime_vals) = match self.selected_function {
                ActivationFunction::ReLU => (relu(&x_vec), relu_prime(&x_vec)),
                ActivationFunction::Sigmoid => (sigmoid(&x_vec), sigmoid_prime(&x_vec)),
                ActivationFunction::Tanh => (tanh(&x_vec), tanh_prime(&x_vec)),
                ActivationFunction::GELU => (gelu(&x_vec), gelu_prime(&x_vec)),
            };

            let points: PlotPoints = x_vals
                .iter()
                .zip(y_vals.iter())
                .map(|(&x, &y)| [x, y])
                .collect();
            let line = Line::new(self.selected_function.name(), points);

            let plot = Plot::new("activation_plot")
                .legend(egui_plot::Legend::default())
                .view_aspect(2.0);

            plot.show(ui, |plot_ui| {
                plot_ui.line(line);

                if self.show_derivative {
                    let prime_points: PlotPoints = x_vals
                        .iter()
                        .zip(y_prime_vals.iter())
                        .map(|(&x, &y)| [x, y])
                        .collect();
                    let prime_line =
                        Line::new(format!("{}'", self.selected_function.name()), prime_points)
                            .style(LineStyle::Dashed { length: 5.0 });
                    plot_ui.line(prime_line);
                }
            });
        });
    }
}
