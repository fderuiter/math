use crate::async_sim::unified::{UnifiedModel, UnifiedSimTool};
use crate::async_sim::{SimCommand, StateSnapshot};
use math_commons::theory::ParameterConstraint;
use math_explorer::physics::quantum::{
    construct_1d_hamiltonian, evolve_state, gaussian_wavepacket, QuantumOperator, QuantumState,
};
use nalgebra::DVector;
use num_complex::Complex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct WaveUnified {
    psi: QuantumState,
    hamiltonian: QuantumOperator,
    potential: DVector<f64>,
    x_axis: Vec<f64>,
    time: f64,
    n_points: usize,
    x_min: f64,
    x_max: f64,
}

impl UnifiedModel for WaveUnified {
    fn new(params: &HashMap<String, f64>) -> Self {
        let n_points = 200;
        let x_min = -5.0;
        let x_max = 5.0;

        let potential_type = *params.get("potential_type").unwrap_or(&0.0);
        let mut sim = Self {
            psi: QuantumState::new(DVector::from_element(n_points, Complex::new(0.0, 0.0))),
            hamiltonian: construct_1d_hamiltonian(&DVector::zeros(n_points), 1.0, 1.0, 1.0),
            potential: DVector::zeros(n_points),
            x_axis: Vec::new(),
            time: 0.0,
            n_points,
            x_min,
            x_max,
        };
        sim.init_system(potential_type);
        sim
    }

    fn step(&mut self, _params: &HashMap<String, f64>) {
        let dt = 0.05;
        self.psi = evolve_state(&self.psi, &self.hamiltonian, dt, 1.0);
        self.time += dt;
        self.psi = self.psi.normalize();
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let prob_density = self.psi.probability_density();
        let mut custom_data = Vec::with_capacity(self.n_points * 2);
        for i in 0..self.n_points {
            custom_data.push(self.x_axis[i]);
            custom_data.push(prob_density[i]);
        }

        StateSnapshot {
            width: 0,
            height: 0,
            pixels: Arc::new(std::sync::RwLock::new(Vec::new())),
            custom_data,
            structured_data: None,
        }
    }

    fn process_command(&mut self, cmd: SimCommand, params: &HashMap<String, f64>) {
        if let SimCommand::Reset = cmd {
            let potential_type = *params.get("potential_type").unwrap_or(&0.0);
            self.init_system(potential_type);
        }
    }

    fn parameters() -> HashMap<String, ParameterConstraint> {
        let mut map = HashMap::new();
        // 0.0 for Infinite Well, 1.0 for Harmonic Oscillator
        map.insert("potential_type".to_string(), ParameterConstraint { min: 0.0, max: 1.0, step: 1.0 });
        map
    }

    fn name() -> &'static str {
        "Wave Simulator"
    }
}

impl WaveUnified {
    fn init_system(&mut self, potential_type: f64) {
        let dx = (self.x_max - self.x_min) / (self.n_points as f64 - 1.0);
        self.x_axis = (0..self.n_points)
            .map(|i| self.x_min + i as f64 * dx)
            .collect();

        self.potential = DVector::zeros(self.n_points);
        for (i, &x) in self.x_axis.iter().enumerate() {
            let v = if potential_type < 0.5 {
                0.0 // InfiniteWell
            } else {
                0.5 * x * x // HarmonicOscillator
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

inventory::submit! {
    crate::framework::ToolMetadata {
        name: "WaveSimulator",
        domain: "quantum",
        tags: &[],
        build: || Box::new(UnifiedSimTool::<WaveUnified>::new()),
    }
}

impl math_commons::theory::TheoryDescribable for WaveUnified {
    fn theory_description(&self) -> String {
        "Quantum wave simulation [cite:quantum_mechanics]".to_string()
    }
    fn phonetic_description(&self) -> String {
        "Quantum wave simulation".to_string()
    }
    fn theory_citation(&self) -> String {
        "[cite:quantum_mechanics]".to_string()
    }
    fn available_descriptions(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}
