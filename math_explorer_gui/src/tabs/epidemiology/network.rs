use super::EpidemiologyTool;
use eframe::egui;
use egui::Color32;
use math_explorer::epidemiology::networks::heterogeneous_r0;
use std::f32::consts::TAU;

pub struct NetworkPropagationTool {
    beta: f64,
    gamma: f64,
    mean_degree: f64,
    degree_variance: f64,
    r0: f64,

    // Fixed graph visualization parameters
    num_nodes: usize,
    positions: Vec<[f32; 2]>,
    edges: Vec<(usize, usize)>,
}

impl Default for NetworkPropagationTool {
    fn default() -> Self {
        let num_nodes = 8;
        let mut positions = Vec::new();
        let radius = 100.0;

        for i in 0..num_nodes {
            let angle = i as f32 * TAU / num_nodes as f32;
            positions.push([angle.cos() * radius, angle.sin() * radius]);
        }

        let edges = vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0),
            (0, 4), (1, 5), (2, 6), (3, 7)
        ];

        let mut tool = Self {
            beta: 0.5,
            gamma: 0.1,
            mean_degree: 3.0,
            degree_variance: 1.0,
            r0: 0.0,
            num_nodes,
            positions,
            edges,
        };
        tool.recalculate();
        tool
    }
}

impl NetworkPropagationTool {
    fn recalculate(&mut self) {
        self.r0 = heterogeneous_r0(
            self.beta,
            self.gamma,
            self.mean_degree,
            self.degree_variance,
        );
    }
}

impl EpidemiologyTool for NetworkPropagationTool {
    fn name(&self) -> &'static str {
        "Network Propagation"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left panel for controls
            ui.vertical(|ui| {
                ui.heading("Parameters");
                let mut changed = false;

                changed |= ui
                    .add(egui::Slider::new(&mut self.beta, 0.0..=1.0).text("Transmission Rate (beta)"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.gamma, 0.01..=1.0).text("Recovery Rate (gamma)"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.mean_degree, 0.1..=10.0).text("Mean Degree <k>"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.degree_variance, 0.0..=10.0).text("Degree Variance Var(k)"))
                    .changed();

                if changed {
                    self.recalculate();
                }

                ui.separator();
                ui.heading("Results");
                ui.label(format!("Calculated R0: {:.2}", self.r0));

                if self.r0 > 1.0 {
                    ui.colored_label(Color32::RED, "Epidemic expected (R0 > 1)");
                } else {
                    ui.colored_label(Color32::GREEN, "Disease dies out (R0 <= 1)");
                }

                ui.separator();
                ui.label("R0 for heterogeneous networks is sensitive to the variance in degree distribution. Higher variance (e.g., superspreaders) can sustain an epidemic even with a low mean degree.");
            });

            ui.separator();

            // Right panel for visualization
            ui.vertical(|ui| {
                ui.heading("Network Visualization");

                let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                let center = response.rect.center();

                // Draw edges
                for &(u, v) in &self.edges {
                    let p1 = center + egui::vec2(self.positions[u][0], self.positions[u][1]);
                    let p2 = center + egui::vec2(self.positions[v][0], self.positions[v][1]);
                    painter.line_segment([p1, p2], (1.0, Color32::GRAY));
                }

                // Draw nodes
                for i in 0..self.num_nodes {
                    let p = center + egui::vec2(self.positions[i][0], self.positions[i][1]);

                    // Assign colors to make it look like a spreading disease
                    let color = if i % 3 == 0 {
                        Color32::RED // Infected
                    } else if i % 5 == 0 {
                        Color32::BLUE // Recovered
                    } else {
                        Color32::LIGHT_GREEN // Susceptible
                    };

                    painter.circle_filled(p, 10.0, color);
                    painter.circle_stroke(p, 10.0, (1.0, Color32::WHITE));
                }
            });
        });
    }
}
