use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use math_explorer::physics::quantum::{
    construct_1d_hamiltonian, evolve_state, gaussian_wavepacket, QuantumOperator, QuantumState,
};
use nalgebra::DVector;
use num_complex::Complex;

use super::QuantumTool;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PotentialType {
    InfiniteWell,
    HarmonicOscillator,
}

pub struct WaveSimulator {
    psi: QuantumState,
    hamiltonian: QuantumOperator,
    potential_type: PotentialType,
    potential: DVector<f64>,
    x_axis: Vec<f64>,
    time: f64,
    paused: bool,

    // Simulation parameters
    n_points: usize,
    x_min: f64,
    x_max: f64,
}

impl Default for WaveSimulator {
    fn default() -> Self {
        let n_points = 200;
        let x_min = -5.0;
        let x_max = 5.0;
        let dx = (x_max - x_min) / (n_points as f64 - 1.0);

        // Initial dummies to satisfy struct initialization
        // They will be overwritten by init_system immediately
        let potential = DVector::zeros(n_points);
        let hamiltonian = construct_1d_hamiltonian(&potential, dx, 1.0, 1.0);
        let psi = QuantumState::new(DVector::from_element(n_points, Complex::new(0.0, 0.0)));

        let mut tab = Self {
            psi,
            hamiltonian,
            potential_type: PotentialType::InfiniteWell,
            potential,
            x_axis: Vec::new(),
            time: 0.0,
            paused: true,
            n_points,
            x_min,
            x_max,
        };

        tab.init_system();
        tab
    }
}

impl WaveSimulator {
    fn init_system(&mut self) {
        let dx = (self.x_max - self.x_min) / (self.n_points as f64 - 1.0);
        self.x_axis = (0..self.n_points)
            .map(|i| self.x_min + i as f64 * dx)
            .collect();

        // 1. Initialize Potential
        self.potential = DVector::zeros(self.n_points);
        for (i, &x) in self.x_axis.iter().enumerate() {
            let v = match self.potential_type {
                PotentialType::InfiniteWell => {
                    // Inside the box (-5 to 5) V=0. The boundaries are the simulation limits.
                    0.0
                }
                PotentialType::HarmonicOscillator => 0.5 * x * x,
            };
            self.potential[i] = v;
        }

        // 2. Construct Hamiltonian H = T + V
        // Using mass = 1.0, h_bar = 1.0
        self.hamiltonian = construct_1d_hamiltonian(&self.potential, dx, 1.0, 1.0);

        // 3. Initialize Wavefunction (Gaussian Packet)
        // Start at x0 = -2.0, moving right with k0 = 5.0
        let x0 = -2.0;
        let k0 = 5.0;
        let sigma = 0.5;

        self.psi = gaussian_wavepacket(&self.x_axis, x0, k0, sigma);
        self.time = 0.0;
    }

    fn step(&mut self, dt: f64) {
        // Evolve state
        self.psi = evolve_state(&self.psi, &self.hamiltonian, dt, 1.0);
        self.time += dt;
        // Re-normalize to prevent drift
        self.psi = self.psi.normalize();
    }
}

impl QuantumTool for WaveSimulator {
    fn name(&self) -> &'static str {
        "Wave Simulator"
    }

    fn show(&mut self, ctx: &egui::Context) {
        // --- Simulation Control ---
        if !self.paused {
            self.step(0.05); // Fixed time step per frame
            ctx.request_repaint();
        }

        egui::SidePanel::left("quantum_controls").show(ctx, |ui| {
            ui.heading("Quantum Control");
            ui.separator();

            ui.label("Potential:");
            let mut changed = false;
            changed |= ui
                .radio_value(
                    &mut self.potential_type,
                    PotentialType::InfiniteWell,
                    "Infinite Well",
                )
                .clicked();
            changed |= ui
                .radio_value(
                    &mut self.potential_type,
                    PotentialType::HarmonicOscillator,
                    "Harmonic Oscillator",
                )
                .clicked();

            if changed {
                self.init_system();
            }

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .button(if self.paused { "▶ Play" } else { "⏸ Pause" })
                    .clicked()
                {
                    self.paused = !self.paused;
                }
                if ui
                    .button("🔄 Reset")
                    .on_hover_text("Reset the simulation to its initial state")
                    .clicked()
                {
                    self.init_system();
                }
            });

            ui.separator();
            ui.label(format!("Time: {:.2}", self.time));
            ui.label("White: |ψ(x)|² (Probability)");
            ui.colored_label(
                egui::Color32::from_rgb(100, 100, 255),
                "Blue: V(x) (Potential)",
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let prob_density = self.psi.probability_density();

            // Plot data
            let psi_points: Vec<[f64; 2]> = self
                .x_axis
                .iter()
                .zip(prob_density.iter())
                .map(|(&x, &p)| [x, p])
                .collect();

            // Scale potential for visualization (e.g. max potential matches max prob)
            // Or just plot raw. V can be large. Let's scale it to fit roughly in 0-1 range if needed.
            // But V depends on x.
            // Let's just plot V scaled by 0.2 to not overwhelm.
            let v_points: Vec<[f64; 2]> = self
                .x_axis
                .iter()
                .zip(self.potential.iter())
                .map(|(&x, &v)| [x, v * 0.2])
                .collect();

            Plot::new("quantum_plot")
                .legend(Legend::default())
                .x_axis_label("Position (x)")
                .y_axis_label("Probability Density / Potential")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("Probability |ψ|²", PlotPoints::new(psi_points))
                            .color(egui::Color32::WHITE)
                            .width(2.0),
                    );
                    plot_ui.line(
                        Line::new("Potential V(x) (scaled)", PlotPoints::new(v_points))
                            .color(egui::Color32::from_rgb(100, 100, 255)),
                    );
                });
        });
    }
}
