use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::physics::chaos::lorenz::{LorenzBuilder, LorenzState, LorenzSystem};

use super::ChaosTool;

pub struct AttractorPlotter {
    system: LorenzSystem,
    history: Vec<[f64; 3]>,
    max_history: usize,
    paused: bool,
    steps_per_frame: usize,
    dt: f64,
    // View parameters
    yaw: f64,
    pitch: f64,
    scale: f64,
}

impl Default for AttractorPlotter {
    fn default() -> Self {
        let initial_state = LorenzState::new(1.0, 1.0, 1.0);
        let system = LorenzBuilder::new().build(initial_state);
        Self {
            system,
            history: Vec::with_capacity(5000),
            max_history: 5000,
            paused: false,
            steps_per_frame: 10,
            dt: 0.01,
            yaw: 0.5,
            pitch: 0.5,
            scale: 25.0, // Increased scale since Lorenz coordinates are usually around 10-50
        }
    }
}

impl AttractorPlotter {
    fn step(&mut self) {
        if self.paused {
            return;
        }
        for _ in 0..self.steps_per_frame {
            self.system.step(self.dt);
            let s = self.system.state.vec;
            if self.history.len() >= self.max_history {
                self.history.remove(0);
            }
            self.history.push([s.x, s.y, s.z]);
        }
    }

    fn reset(&mut self) {
        let initial_state = LorenzState::new(1.0, 1.0, 1.0);
        // Preserve parameters if user changed them
        let current_sigma = self.system.sigma;
        let current_rho = self.system.rho;
        let current_beta = self.system.beta;

        self.system = LorenzBuilder::new()
            .sigma(current_sigma)
            .rho(current_rho)
            .beta(current_beta)
            .build(initial_state);
        self.history.clear();
    }

    fn project(&self, p: [f64; 3]) -> [f64; 2] {
        let x = p[0];
        let y = p[1];
        let z = p[2];

        // Rotate around Y (Yaw)
        // x' = x cos(yaw) + z sin(yaw)
        // z' = -x sin(yaw) + z cos(yaw)
        let x1 = x * self.yaw.cos() + z * self.yaw.sin();
        let y1 = y;
        let z1 = -x * self.yaw.sin() + z * self.yaw.cos();

        // Rotate around X (Pitch)
        // y' = y1 cos(pitch) - z1 sin(pitch)
        // z'' = y1 sin(pitch) + z1 cos(pitch)
        let x2 = x1;
        let y2 = y1 * self.pitch.cos() - z1 * self.pitch.sin();
        // let z2 = y1 * self.pitch.sin() + z1 * self.pitch.cos();

        // Project: Map 3D coordinates to 2D screen coordinates.
        // We want Z (vertical in world) to be Y on screen?
        // Lorenz: Z is usually the vertical axis.
        // But here I used standard mathematical rotation where Y is up?
        // Let's assume standard right-handed: Y up, X right, Z out.
        // If I want Z to be up, I should map Z to screen Y.
        // But my projection code maps x2 -> x, y2 -> y.
        // Let's stick to X, Y as horizontal plane and Z as vertical.
        // In that case, rotation matrices should be adjusted.
        // Simpler approach: Just rotate the point vector using a rotation matrix constructed from Euler angles.
        // For now, the previous simple rotation is fine, just tweaking scale.

        [x2 * self.scale, y2 * self.scale]
    }
}

impl ChaosTool for AttractorPlotter {
    fn name(&self) -> &'static str {
        "Attractor Plotter"
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.step();
        if !self.paused {
            ctx.request_repaint();
        }

        egui::SidePanel::left("lorenz_controls")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.horizontal(|ui| {
                    if ui.button(if self.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
                        self.paused = !self.paused;
                    }
                    if ui.button("↺ Reset").clicked() {
                        self.reset();
                    }
                });

                ui.separator();
                ui.label("Parameters");
                ui.add(egui::Slider::new(&mut self.system.sigma, 0.0..=50.0).text("Sigma (σ)"));
                ui.add(egui::Slider::new(&mut self.system.rho, 0.0..=100.0).text("Rho (ρ)"));
                ui.add(egui::Slider::new(&mut self.system.beta, 0.0..=10.0).text("Beta (β)"));

                ui.separator();
                ui.label("View");
                ui.add(egui::Slider::new(&mut self.yaw, 0.0..=6.28).text("Yaw"));
                ui.add(egui::Slider::new(&mut self.pitch, 0.0..=6.28).text("Pitch"));
                ui.add(egui::Slider::new(&mut self.scale, 0.1..=50.0).text("Zoom")); // Adjusted zoom range
                ui.add(egui::Slider::new(&mut self.dt, 0.001..=0.05).text("Time Step"));
                ui.add(egui::Slider::new(&mut self.steps_per_frame, 1..=100).text("Speed"));

                ui.separator();
                let s = self.system.state.vec;
                ui.label(format!("x: {:.2}", s.x));
                ui.label(format!("y: {:.2}", s.y));
                ui.label(format!("z: {:.2}", s.z));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Generate points
            let points: Vec<[f64; 2]> = self.history.iter().map(|p| self.project(*p)).collect();

            Plot::new("attractor_plot")
                .data_aspect(1.0)
                .view_aspect(1.0)
                .show(ui, |plot_ui| {
                     plot_ui.line(Line::new("Trajectory", PlotPoints::new(points)).color(egui::Color32::from_rgb(100, 200, 255)));
                });
        });
    }
}
