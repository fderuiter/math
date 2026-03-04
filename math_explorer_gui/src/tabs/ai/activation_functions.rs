use crate::tabs::ai::AiTool;
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use math_explorer::ai::deep_learning_theory::calculus::{
    gelu, gelu_prime, relu, relu_prime, sigmoid, sigmoid_prime, tanh, tanh_prime,
};
use math_explorer::ai::deep_learning_theory::linear_algebra::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActivationFunction {
    ReLU,
    Sigmoid,
    Tanh,
    GELU,
}

pub struct ActivationFunctionsTool {
    selected_function: ActivationFunction,
    x_min: f64,
    x_max: f64,
    points: usize,
}

impl Default for ActivationFunctionsTool {
    fn default() -> Self {
        Self {
            selected_function: ActivationFunction::ReLU,
            x_min: -5.0,
            x_max: 5.0,
            points: 200,
        }
    }
}

impl AiTool for ActivationFunctionsTool {
    fn name(&self) -> &'static str {
        "Activation Functions"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("activation_functions_controls").show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.label("Select Activation Function:");
            ui.radio_value(&mut self.selected_function, ActivationFunction::ReLU, "ReLU");
            ui.radio_value(
                &mut self.selected_function,
                ActivationFunction::Sigmoid,
                "Sigmoid",
            );
            ui.radio_value(&mut self.selected_function, ActivationFunction::Tanh, "Tanh");
            ui.radio_value(&mut self.selected_function, ActivationFunction::GELU, "GELU");

            ui.separator();
            ui.label("Plot Range");
            ui.horizontal(|ui| {
                ui.label("X Min:");
                ui.add(egui::DragValue::new(&mut self.x_min).speed(0.1));
            });
            ui.horizontal(|ui| {
                ui.label("X Max:");
                ui.add(egui::DragValue::new(&mut self.x_max).speed(0.1));
            });

            // Ensure min < max
            if self.x_min >= self.x_max {
                self.x_max = self.x_min + 1.0;
            }

            ui.separator();
            ui.label("Math Info");
            match self.selected_function {
                ActivationFunction::ReLU => {
                    ui.label("f(x) = max(0, x)");
                    ui.label("f'(x) = 1 if x > 0 else 0");
                }
                ActivationFunction::Sigmoid => {
                    ui.label("f(x) = 1 / (1 + e^(-x))");
                    ui.label("f'(x) = f(x) * (1 - f(x))");
                }
                ActivationFunction::Tanh => {
                    ui.label("f(x) = tanh(x)");
                    ui.label("f'(x) = 1 - tanh^2(x)");
                }
                ActivationFunction::GELU => {
                    ui.label("f(x) ≈ 0.5 * x * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3)))");
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Generate x values
            let step = (self.x_max - self.x_min) / (self.points as f64 - 1.0);
            let mut x_vals = Vec::with_capacity(self.points);
            for i in 0..self.points {
                x_vals.push(self.x_min + (i as f64) * step);
            }

            let x_vec = Vector::from_vec(x_vals.clone());

            // Compute y values and derivatives
            let (y_vec, dy_vec) = match self.selected_function {
                ActivationFunction::ReLU => (relu(&x_vec), relu_prime(&x_vec)),
                ActivationFunction::Sigmoid => (sigmoid(&x_vec), sigmoid_prime(&x_vec)),
                ActivationFunction::Tanh => (tanh(&x_vec), tanh_prime(&x_vec)),
                ActivationFunction::GELU => (gelu(&x_vec), gelu_prime(&x_vec)),
            };

            // Map to egui_plot points
            let plot_points: PlotPoints = x_vals
                .iter()
                .zip(y_vec.iter())
                .map(|(x, y)| [*x, *y])
                .collect();

            let deriv_points: PlotPoints = x_vals
                .iter()
                .zip(dy_vec.iter())
                .map(|(x, y)| [*x, *y])
                .collect();

            let line = Line::new("f(x)", plot_points).width(2.0);
            let deriv_line = Line::new("f'(x)", deriv_points).width(1.5);

            Plot::new("activation_function_plot")
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                    plot_ui.line(deriv_line);
                });
        });
    }
}
