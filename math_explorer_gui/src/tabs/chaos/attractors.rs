use crate::accessibility::AccessibleTheoryHover;
use crate::framework::InteractiveTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::physics::chaos::lorenz::{LorenzBuilder, LorenzState, LorenzSystem};
use nalgebra::Vector3;
use std::collections::VecDeque;

pub struct AttractorPlotter {
    system: LorenzSystem,

    // Simulation Control
    paused: bool,
    simulation_speed: usize,
    dt: f64,

    // Visualization
    history: VecDeque<Vector3<f64>>, // Store as Vector3 for easier math
    max_points: usize,

    camera: crate::framework::Camera3D,
}

impl Default for AttractorPlotter {
    fn default() -> Self {
        let initial_state = LorenzState::new(1.0, 1.0, 1.0);
        let system = LorenzBuilder::new().build(initial_state);

        Self {
            system,
            paused: false,
            simulation_speed: 5,
            dt: 0.01,
            history: VecDeque::with_capacity(2000),
            max_points: 2000,
            camera: crate::framework::Camera3D::new(0.0, 0.0, 1.0),
        }
    }
}

impl AttractorPlotter {
    fn reset(&mut self) {
        let initial_state = LorenzState::new(1.0, 1.0, 1.0);
        // Preserve parameters but reset state
        let sigma = self.system.sigma;
        let rho = self.system.rho;
        let beta = self.system.beta;

        self.system = LorenzBuilder::new()
            .sigma(sigma)
            .rho(rho)
            .beta(beta)
            .build(initial_state);

        self.history.clear();
    }

    /// Projects 3D point to 2D screen space based on rotation
    fn project(&self, p: Vector3<f64>) -> [f64; 2] {
        // Center the attractor roughly. Lorenz attractor Z ranges approx 0-50.
        // Centering it makes rotation look more natural.
        let center_offset = Vector3::new(0.0, 0.0, 25.0);
        let p_centered = p - center_offset;

        self.camera.project(&[p_centered.x, p_centered.y, p_centered.z])
    }
}

impl InteractiveTool for AttractorPlotter {
    fn theory(&self) -> &dyn math_commons::theory::TheoryDescribable { self }
    fn name(&self) -> &'static str {
        "Attractor Plotter"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        // --- Simulation ---
        if !self.paused {
            for _ in 0..self.simulation_speed {
                self.system.step(self.dt);
                if self.history.len() >= self.max_points {
                    self.history.pop_front();
                }
                self.history.push_back(self.system.state.vec);
            }
            ctx.request_repaint();
        }

        // --- UI ---
        egui::SidePanel::left("attractor_controls").show(ctx, |ui| {
            ui.heading("Lorenz Attractor");
            ui.separator();

            ui.collapsing("Parameters", |ui| {
                ui.add(egui::Slider::new(&mut self.system.sigma, 0.0..=50.0).text("Prandtl Number (σ)"));

                ui.add(egui::Slider::new(&mut self.system.rho, 0.0..=100.0).text("Rayleigh Number (ρ)"));

                ui.add(egui::Slider::new(&mut self.system.beta, 0.0..=10.0).text("Geometric Factor (β)"));
            });

            ui.collapsing("Simulation", |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(if self.paused { "▶ Play" } else { "⏸ Pause" })
                        .clicked()
                    {
                        self.paused = !self.paused;
                    }
                    if ui.button("↻ Reset").clicked() {
                        self.reset();
                    }
                });

                ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=50).text("Speed (steps/frame)"));

                ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.05).text("Time Step (dt)"));

                ui.add(egui::Slider::new(&mut self.max_points, 100..=10000).text("Max Points"));
            });

            ui.collapsing("View", |ui| {
                self.camera.ui(ui);
            });

            ui.separator();
            let state = self.system.state.vec;
            ui.label(format!("X: {:.2}", state.x));
            ui.label(format!("Y: {:.2}", state.y));
            ui.label(format!("Z: {:.2}", state.z));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let points: Vec<[f64; 2]> = self.history.iter().map(|p| self.project(*p)).collect();

            let response = Plot::new("attractor_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("Trajectory", PlotPoints::new(points))
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                })
                .response;
            
            self.camera.handle_interaction(&response, ui);
            
            response.accessible_theory_hover(&self.system);

            ui.label("Drag 'Yaw' and 'Pitch' in the side panel to rotate the view.");
        });
    }
}

// [cite:chaos]


inventory::submit! {
    crate::framework::ToolMetadata {
        name: "AttractorPlotter",
        domain: "chaos",
        tags: &[],
        build: || Box::new(AttractorPlotter::default()),
    }
}

impl math_commons::theory::TheoryDescribable for AttractorPlotter {
    fn theory_description(&self) -> String { "Theoretical context not available.".into() }
    fn phonetic_description(&self) -> String { "Theoretical context not available.".into() }
    fn theory_citation(&self) -> String { "Uncited".into() }
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> { std::collections::HashMap::new() }
}
