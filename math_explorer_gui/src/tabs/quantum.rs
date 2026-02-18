use crate::tabs::ExplorerTab;
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use math_explorer::physics::quantum::{evolve_state, QuantumOperator, QuantumState};
use nalgebra::{DMatrix, DVector};
use num_complex::Complex;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PotentialType {
    InfiniteWell,
    HarmonicOscillator,
}

pub struct QuantumTab {
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

impl Default for QuantumTab {
    fn default() -> Self {
        let n_points = 200;
        let x_min = -5.0;
        let x_max = 5.0;

        let mut tab = Self {
            psi: QuantumState::new(DVector::from_element(n_points, Complex::new(0.0, 0.0))),
            hamiltonian: QuantumOperator::new(DMatrix::zeros(n_points, n_points)),
            potential_type: PotentialType::InfiniteWell,
            potential: DVector::zeros(n_points),
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

impl QuantumTab {
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
        // T = -h_bar^2 / 2m * d^2/dx^2
        // Finite difference: d^2/dx^2 ~ (psi[i+1] - 2psi[i] + psi[i-1]) / dx^2
        // With h_bar = 1, m = 1 -> coeff = -1 / (2 * dx^2)
        // let coeff = -1.0 / (2.0 * dx * dx);

        let kin_diag = 1.0 / (dx * dx);
        let kin_off = -0.5 / (dx * dx);

        let mut h_matrix = DMatrix::<Complex<f64>>::zeros(self.n_points, self.n_points);

        for i in 0..self.n_points {
            // Kinetic Energy
            h_matrix[(i, i)] = Complex::new(kin_diag, 0.0);
            if i > 0 {
                h_matrix[(i, i - 1)] = Complex::new(kin_off, 0.0);
            }
            if i < self.n_points - 1 {
                h_matrix[(i, i + 1)] = Complex::new(kin_off, 0.0);
            }

            // Potential Energy
            h_matrix[(i, i)] += Complex::new(self.potential[i], 0.0);
        }

        self.hamiltonian = QuantumOperator::new(h_matrix);

        // 3. Initialize Wavefunction (Gaussian Packet)
        // Start at x0 = -2.0, moving right with k0 = 5.0
        let x0 = -2.0;
        let k0 = 5.0;
        let sigma = 0.5;
        let normalization = 1.0 / (sigma * (PI).sqrt()).sqrt(); // roughly

        let mut psi_vec = DVector::<Complex<f64>>::zeros(self.n_points);
        for (i, &x) in self.x_axis.iter().enumerate() {
            let gauss = (-((x - x0).powi(2)) / (2.0 * sigma * sigma)).exp();
            let plane_wave = Complex::new(0.0, k0 * x).exp();
            psi_vec[i] = Complex::new(normalization * gauss, 0.0) * plane_wave;
        }

        self.psi = QuantumState::new(psi_vec).normalize();
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

impl ExplorerTab for QuantumTab {
    fn name(&self) -> &'static str {
        "Quantum Mechanics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                if ui.button("↺ Reset").clicked() {
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
