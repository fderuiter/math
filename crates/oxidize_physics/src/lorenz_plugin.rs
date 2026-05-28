use oxidize_core::plugin::{DynamicSimulation, Plugin};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LorenzConfig {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
    pub dt: f64,
}

impl oxidize_core::ModelConfig for LorenzConfig {}

#[derive(Clone, Serialize, Deserialize)]
pub struct LorenzState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl oxidize_core::ModelState for LorenzState {}

pub struct LorenzSimulation {
    config: LorenzConfig,
    state: LorenzState,
}

#[derive(Debug, thiserror::Error)]
#[error("Lorenz Error: {0}")]
pub struct LorenzError(String);

impl oxidize_core::SimulationModel for LorenzSimulation {
    type Config = LorenzConfig;
    type State = LorenzState;
    type Error = LorenzError;

    fn initialize(config: Self::Config) -> Result<Self, Self::Error> {
        Ok(Self {
            config,
            state: LorenzState { x: 1.0, y: 1.0, z: 1.0 },
        })
    }

    fn step(&mut self) -> Result<(), Self::Error> {
        let dx = self.config.sigma * (self.state.y - self.state.x);
        let dy = self.state.x * (self.config.rho - self.state.z) - self.state.y;
        let dz = self.state.x * self.state.y - self.config.beta * self.state.z;

        self.state.x += dx * self.config.dt;
        self.state.y += dy * self.config.dt;
        self.state.z += dz * self.config.dt;
        Ok(())
    }

    fn get_state(&self) -> Self::State {
        self.state.clone()
    }
}

pub struct LorenzPlugin;

impl Plugin for LorenzPlugin {
    fn name(&self) -> &'static str {
        "Lorenz Attractor"
    }

    fn description(&self) -> &'static str {
        "A simple model of atmospheric convection."
    }

    fn get_default_config_json(&self) -> String {
        serde_json::to_string_pretty(&LorenzConfig {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            dt: 0.01,
        }).unwrap()
    }

    fn initialize_from_json(&self, json: &str) -> Result<Box<dyn DynamicSimulation>, String> {
        use oxidize_core::SimulationModel;
        let config: LorenzConfig = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let sim = LorenzSimulation::initialize(config).map_err(|e| e.to_string())?;
        Ok(Box::new(sim))
    }
}

impl DynamicSimulation for LorenzSimulation {
    fn step(&mut self) -> Result<(), String> {
        <Self as oxidize_core::SimulationModel>::step(self).map_err(|e| e.to_string())
    }

    fn get_state_json(&self) -> String {
        use oxidize_core::SimulationModel;
        serde_json::to_string_pretty(&self.get_state()).unwrap()
    }
}

inventory::submit! {
    &LorenzPlugin as &'static dyn Plugin
}
