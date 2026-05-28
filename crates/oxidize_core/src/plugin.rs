use crate::{ModelConfig, ModelState, SimulationModel};

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn get_default_config_json(&self) -> String;
    fn initialize_from_json(&self, json: &str) -> Result<Box<dyn DynamicSimulation>, String>;
}

pub trait DynamicSimulation: Send + Sync {
    fn step(&mut self) -> Result<(), String>;
    fn get_state_json(&self) -> String;
}

inventory::collect!(&'static dyn Plugin);
