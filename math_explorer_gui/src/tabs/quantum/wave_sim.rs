use crate::async_sim::declarative::{DeclarativeSimulation, DeclarativeTab, PlotData, PlotLine};
use crate::declare_params;
use crate::async_sim::StateSnapshot;
use crate::tabs::ExplorerTab;
use eframe::egui;
use math_explorer::physics::quantum::{
    construct_1d_hamiltonian, evolve_state, gaussian_wavepacket, QuantumOperator, QuantumState,
};
use nalgebra::DVector;
use num_complex::Complex;
use std::any::Any;

use super::QuantumTool;

declare_params! {
    pub struct WaveParams {
        #[param(name = "Potential Type (0=Well, 1=Osc)", min = 0.0, max = 1.0)]
        pub potential_type: f64,
    }
}

pub struct WaveRunner {
    psi: QuantumState,
    hamiltonian: QuantumOperator,
    potential: DVector<f64>,
    x_axis: Vec<f64>,
    time: f64,
    n_points: usize,
    x_min: f64,
    x_max: f64,
}

impl Default for WaveRunner {
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
            potential,
            x_axis: Vec::new(),
            time: 0.0,
            n_points,
            x_min,
            x_max,
        };
        runner.init_system(0.0);
        runner
    }
}

impl WaveRunner {
    fn init_system(&mut self, potential_type: f64) {
        let dx = (self.x_max - self.x_min) / (self.n_points as f64 - 1.0);
        self.x_axis = (0..self.n_points)
            .map(|i| self.x_min + i as f64 * dx)
            .collect();

        self.potential = DVector::zeros(self.n_points);
        for (i, &x) in self.x_axis.iter().enumerate() {
            let v = if potential_type < 0.5 {
                0.0 // Infinite Well (boundaries handled by dirichlet)
            } else {
                0.5 * x * x // Harmonic Oscillator
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

impl DeclarativeSimulation for WaveRunner {
    type Params = WaveParams;

    fn name(&self) -> &'static str {
        "Wave Simulator"
    }

    fn default_params(&self) -> Self::Params {
        WaveParams {
            potential_type: 0.0,
        }
    }

    fn param_descriptors(&self) -> Vec<crate::async_sim::declarative::ParamDescriptor<Self::Params>> {
        WaveParams::descriptors()
    }

    fn setup(&mut self, params: &Self::Params) {
        self.init_system(params.potential_type);
    }

    fn step(&mut self, _params: &Self::Params) {
        let dt = 0.05;
        self.psi = evolve_state(&self.psi, &self.hamiltonian, dt, 1.0);
        self.time += dt;
        self.psi = self.psi.normalize();
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let prob_density = self.psi.probability_density();
        
        let mut psi_points = Vec::with_capacity(self.n_points);
        let mut v_points = Vec::with_capacity(self.n_points);
        
        for i in 0..self.n_points {
            let x = self.x_axis[i];
            let p = prob_density[i];
            let v = self.potential[i];
            psi_points.push([x, p]);
            v_points.push([x, v * 0.2]); // scale potential down for plotting
        }

        let plot_data = PlotData {
            lines: vec![
                PlotLine {
                    name: "Probability |ψ|²".to_string(),
                    points: psi_points,
                    color: [255, 255, 255],
                },
                PlotLine {
                    name: "Potential V(x) (scaled)".to_string(),
                    points: v_points,
                    color: [100, 100, 255],
                },
            ],
        };

        StateSnapshot {
            width: 0,
            height: 0,
            pixels: std::sync::Arc::new(Vec::new()),
            custom_data: Vec::new(),
            structured_data: Some(Box::new(plot_data) as Box<dyn Any + Send>),
        }
    }
}

pub struct WaveSimulator {
    inner: DeclarativeTab<WaveRunner>,
}

impl Default for WaveSimulator {
    fn default() -> Self {
        Self {
            inner: DeclarativeTab::new(WaveRunner::default(), 1),
        }
    }
}

impl QuantumTool for WaveSimulator {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.inner.show_ctx(ctx);
    }
}
