use super::EpidemiologyTool;
use eframe::egui;
use egui::Color32;
use math_explorer::epidemiology::networks::{NetworkEpidemicModel, NodeState};

pub struct NetworkPropagationTool {
    model: NetworkEpidemicModel,
    is_running: bool,
}

impl Default for NetworkPropagationTool {
    fn default() -> Self {
        Self {
            model: NetworkEpidemicModel::new(50, 0.05, 0.02),
            is_running: false,
        }
    }
}

impl NetworkPropagationTool {
    fn reset_network(&mut self) {
        let mut rng = rand::thread_rng();
        self.model.initialize_geometric_graph_with_rng(&mut rng);
        self.is_running = false;
    }
}

impl EpidemiologyTool for NetworkPropagationTool {
    fn name(&self) -> &'static str {
        "Network Propagation"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        if self.is_running {
            let mut rng = rand::thread_rng();
            self.model.step_with_rng(&mut rng);
            ui.ctx().request_repaint();
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Controls");
                ui.separator();

                if ui
                    .button(if self.is_running {
                        "⏸ Pause"
                    } else {
                        "▶ Start"
                    })
                    .clicked()
                {
                    self.is_running = !self.is_running;
                }
                if ui.button("↺ Reset Network").clicked() {
                    self.reset_network();
                }

                ui.separator();
                let mut changed = false;
                changed |= ui
                    .add(egui::Slider::new(&mut self.model.num_nodes, 10..=200).text("Nodes"))
                    .changed();
                if changed {
                    self.reset_network();
                }
                ui.add(
                    egui::Slider::new(&mut self.model.beta, 0.0..=1.0).text("Transmission (beta)"),
                );
                ui.add(
                    egui::Slider::new(&mut self.model.gamma, 0.0..=1.0).text("Recovery (gamma)"),
                );

                ui.separator();
                // Statistics
                let s_count = self
                    .model
                    .states
                    .iter()
                    .filter(|&&s| s == NodeState::Susceptible)
                    .count();
                let i_count = self
                    .model
                    .states
                    .iter()
                    .filter(|&&s| s == NodeState::Infected)
                    .count();
                let r_count = self
                    .model
                    .states
                    .iter()
                    .filter(|&&s| s == NodeState::Recovered)
                    .count();

                ui.label(
                    egui::RichText::new(format!("Susceptible: {}", s_count)).color(Color32::BLUE),
                );
                ui.label(egui::RichText::new(format!("Infected: {}", i_count)).color(Color32::RED));
                ui.label(
                    egui::RichText::new(format!("Recovered: {}", r_count)).color(Color32::GREEN),
                );
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("Network Visualization");
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                // Draw edges
                for i in 0..self.model.num_nodes {
                    for &j in &self.model.adjacency[i] {
                        if i < j {
                            let p1 = center
                                + egui::vec2(
                                    self.model.positions[i][0],
                                    self.model.positions[i][1],
                                );
                            let p2 = center
                                + egui::vec2(
                                    self.model.positions[j][0],
                                    self.model.positions[j][1],
                                );
                            painter.line_segment([p1, p2], (1.0, Color32::from_gray(100)));
                        }
                    }
                }

                // Draw nodes
                for i in 0..self.model.num_nodes {
                    let p =
                        center + egui::vec2(self.model.positions[i][0], self.model.positions[i][1]);
                    let color = match self.model.states[i] {
                        NodeState::Susceptible => Color32::BLUE,
                        NodeState::Infected => Color32::RED,
                        NodeState::Recovered => Color32::GREEN,
                    };
                    painter.circle_filled(p, 5.0, color);
                    painter.circle_stroke(p, 5.0, (1.0, Color32::WHITE));
                }
            });
        });
    }
}
