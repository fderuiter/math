use crate::config::ModelConfig;
use crate::state::ModelState;

pub trait Simulation: Send + Sync {
    type Config: ModelConfig;
    type State: ModelState;

    fn init(&self, config: &Self::Config) -> Self::State;
    fn step(&self, state: &mut Self::State, config: &Self::Config, dt: f64, input: Option<f64>);
}
