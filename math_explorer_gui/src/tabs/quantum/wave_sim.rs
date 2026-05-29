use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use physics::quantum::{
    construct_1d_hamiltonian, evolve_state, gaussian_wavepacket, QuantumOperator, QuantumState,
};
use nalgebra::DVector;
use num_complex::Complex;
use std::any::Any;

use super::QuantumTool;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PotentialType {
    InfiniteWell,
    HarmonicOscillator,
}

pub struct WaveData {
    pub time: f64,
    pub x_axis: Vec<f64>,
    pub prob_density: Vec<f64>,
    pub potential: Vec<f64>,
}

struct WaveRunner {
    psi: QuantumState,
    hamiltonian: QuantumOperator,
    potential_type: PotentialType,
    potential: DVector<f64>,
    x_axis: Vec<f64>,
    time: f64,
    n_points: usize,
    x_min: f64,
    x_max: f64,
    steps_per_frame: usize,
}

impl SimulationRunner for WaveRunner {
    fn process_command(&mut self, cmd: SimCommand) {
        match cmd {
            SimCommand::SetSpeed(speed) => self.steps_per_frame = speed,
            SimCommand::UpdateParam(name, val) => match name.as_str() {
                "potential_type" => {
                    self.potential_type = if val == 0.0 {
                        PotentialType::InfiniteWell
                    } else {
                        PotentialType::HarmonicOscillator
                    };
                    self.init_system();
                }
                _ => {}
            },
            SimCommand::Reset => self.init_system(),
            _ => {}
        }
    }

    fn step(&mut self) {
        let dt = 0.05;
        self.psi = evolve_state(&self.psi, &self.hamiltonian, dt, 1.0);
        self.time += dt;
        self.psi = self.psi.normalize();
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let prob_density = self.psi.probability_density();
        
        let structured_data = Box::new(WaveData {
            time: self.time,
            x_axis: self.x_axis.clone(),
            prob_density: prob_density.iter().copied().collect(),
            potential: self.potential.iter().copied().collect(),
        }) as Box<dyn Any + Send>;

        StateSnapshot {
            width: 0,
            height: 0,
            pixels: std::sync::Arc::new(Vec::new()),
            custom_data: Vec::new(),
            structured_data: Some(structured_data),
        }
    }

    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

impl WaveRunner {
    fn init_system(&mut self) {
        let dx = (self.x_max - self.x_min) / (self.n_points as f64 - 1.0);
        self.x_axis = (0..self.n_points)
            .map(|i| self.x_min + i as f64 * dx)
            .collect();

        self.potential = DVector::zeros(self.n_points);
        for (i, &x) in self.x_axis.iter().enumerate() {
            let v = match self.potential_type {
                PotentialType::InfiniteWell => 0.0,
                PotentialType::HarmonicOscillator => 0.5 * x * x,
            };
            self.potential[i] = v;
        }

        self.hamiltonian = construct_1d_hamiltonian(&self.potential, dx, 1.0, 1.0);

        let x0 = -2.0;
        let k0 = 5.0;
        let sigma = 0.5;

        self.psi = gaussian_wavepacket(&self.x_axis, x0, k0, sigma);
        self.time = 0.0;
    }
}

pub struct WaveSimulator {
    controller: SimulationController,
    potential_type: PotentialType,
    cached_x: Vec<f64>,
    cached_prob: Vec<f64>,
    cached_pot: Vec<f64>,
    cached_time: f64,
}

impl Default for WaveSimulator {
    fn default() -> Self {
        let n_points = 200;
        let x_min = -5.0;
        let x_max = 5.0;

        let potential = DVector::zeros(n_points);
        let hamiltonian = construct_1d_hamiltonian(&potential, 1.0, 1.0, 1.0);
        let psi = QuantumState::new(DVector::from_element(n_points, Complex::new(0.0, 0.0)));

        let mut runner = WaveRunner {
            psi,
            hamiltonian,
            potential_type: PotentialType::InfiniteWell,
            potential,
            x_axis: Vec::new(),
            time: 0.0,
            n_points,
            x_min,
            x_max,
            steps_per_frame: 1,
        };
        runner.init_system();
        
        let controller = SimulationController::new(runner);

        Self {
            controller,
            potential_type: PotentialType::InfiniteWell,
            cached_x: Vec::new(),
            cached_prob: Vec::new(),
            cached_pot: Vec::new(),
            cached_time: 0.0,
        }
    }
}

impl QuantumTool for WaveSimulator {
    fn name(&self) -> &'static str {
        "Wave Simulator"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        if self.controller.running {
            ctx.request_repaint();
        }
        
        if let Some(snap) = self.controller.update() {
            if let Some(any_data) = &snap.structured_data {
                if let Some(wave_data) = any_data.downcast_ref::<WaveData>() {
                    self.cached_time = wave_data.time;
                    self.cached_x = wave_data.x_axis.clone();
                    self.cached_prob = wave_data.prob_density.clone();
                    self.cached_pot = wave_data.potential.clone();
                }
            }
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
                let val = if self.potential_type == PotentialType::InfiniteWell { 0.0 } else { 1.0 };
                self.controller.send_command(SimCommand::UpdateParam("potential_type".to_string(), val));
                self.controller.send_command(SimCommand::Reset);
            }

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .button(if !self.controller.running { "▶ Play" } else { "⏸ Pause" })
                    .clicked()
                {
                    if self.controller.running {
                        self.controller.send_command(SimCommand::Pause);
                    } else {
                        self.controller.send_command(SimCommand::Start);
                    }
                }
                if ui.button("↻ Reset").clicked() {
                    self.controller.send_command(SimCommand::Reset);
                }
            });

            ui.separator();
            ui.label(format!("Time: {:.2}", self.cached_time));
            ui.label("White: |ψ(x)|² (Probability)");
            ui.colored_label(
                egui::Color32::from_rgb(100, 100, 255),
                "Blue: V(x) (Potential)",
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut psi_points = Vec::new();
            let mut v_points = Vec::new();
            
            for i in 0..self.cached_x.len() {
                let x = self.cached_x[i];
                let p = self.cached_prob[i];
                let v = self.cached_pot[i];
                psi_points.push([x, p]);
                v_points.push([x, v * 0.2]);
            }

            Plot::new("quantum_plot")
                .legend(Legend::default())
                .x_axis_label("Position (x)")
                .y_axis_label("Probability Density / Potential")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("Probability |ψ|²", PlotPoints::new(psi_points))
                            .color(egui::Color32::WHITE)
                            .width(2.0_f32),
                    );
                    plot_ui.line(
                        Line::new("Potential V(x) (scaled)", PlotPoints::new(v_points))
                            .color(egui::Color32::from_rgb(100, 100, 255)),
                    );
                });
        });
    }
}
