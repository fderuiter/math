use super::ChaosTool;
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

    // Camera / Projection
    pitch: f32,
    yaw: f32,
    zoom: f32,
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
            pitch: 0.0,
            yaw: 0.0,
            zoom: 1.0,
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

        // Rotation Matrix
        // Yaw (around Y)
        let cy = (self.yaw as f64).cos();
        let sy = (self.yaw as f64).sin();
        let x1 = p_centered.x * cy - p_centered.z * sy;
        let z1 = p_centered.x * sy + p_centered.z * cy;
        let y1 = p_centered.y;

        // Pitch (around X)
        let cp = (self.pitch as f64).cos();
        let sp = (self.pitch as f64).sin();
        let y2 = y1 * cp - z1 * sp;
        // let z2 = y1 * sp + z1 * cp; // Depth

        // Apply zoom
        [x1 * (self.zoom as f64), y2 * (self.zoom as f64)]
    }
}

impl ChaosTool for AttractorPlotter {
    fn name(&self) -> &'static str {
        "Attractor Plotter"
    }

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
                ui.label("Prandtl Number (σ)");
                ui.add(egui::Slider::new(&mut self.system.sigma, 0.0..=50.0));

                ui.label("Rayleigh Number (ρ)");
                ui.add(egui::Slider::new(&mut self.system.rho, 0.0..=100.0));

                ui.label("Geometric Factor (β)");
                ui.add(egui::Slider::new(&mut self.system.beta, 0.0..=10.0));
            });

            ui.collapsing("Simulation", |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(if self.paused { "▶ Play" } else { "⏸ Pause" })
                        .clicked()
                    {
                        self.paused = !self.paused;
                    }
                    if ui
                        .button("↻ Reset")
                        .on_hover_text("Reset the simulation to its initial state")
                        .clicked()
                    {
                        self.reset();
                    }
                });

                ui.label("Speed (steps/frame)");
                ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=50));

                ui.label("Time Step (dt)");
                ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.05));

                ui.label("Max Points");
                ui.add(egui::Slider::new(&mut self.max_points, 100..=10000));
            });

            ui.collapsing("View", |ui| {
                ui.label("Yaw (Rotate Y)");
                ui.drag_angle(&mut self.yaw);

                ui.label("Pitch (Rotate X)");
                ui.drag_angle(&mut self.pitch);

                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0));
            });

            ui.separator();
            let state = self.system.state.vec;
            ui.label(format!("X: {:.2}", state.x));
            ui.label(format!("Y: {:.2}", state.y));
            ui.label(format!("Z: {:.2}", state.z));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let points: Vec<[f64; 2]> = self.history.iter().map(|p| self.project(*p)).collect();

            Plot::new("attractor_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("Trajectory", PlotPoints::new(points))
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                });

            ui.label("Drag 'Yaw' and 'Pitch' in the side panel to rotate the view.");
        });
    }
}
