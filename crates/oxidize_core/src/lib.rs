use serde::{Deserialize, Serialize};

pub trait ModelConfig: Clone + Serialize + Deserialize<'static> {}

pub trait ModelState: Clone {}

pub trait SimulationModel: Sized {
    type Config: ModelConfig;
    type State: ModelState;
    type Error: std::error::Error + Send + Sync + 'static;

    fn initialize(config: Self::Config) -> Result<Self, Self::Error>;
    fn step(&mut self) -> Result<(), Self::Error>;
    fn get_state(&self) -> Self::State;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    pub struct BooleanMaskConfig {
        pub width: usize,
        pub height: usize,
    }

    impl ModelConfig for BooleanMaskConfig {}

    #[derive(Clone)]
    pub struct BooleanMaskState {
        pub grid: Vec<bool>,
        pub width: usize,
        pub height: usize,
    }

    impl ModelState for BooleanMaskState {}

    pub struct BooleanMaskSimulation {
        config: BooleanMaskConfig,
        state: BooleanMaskState,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum SimulationError {
        #[error("Initialization error")]
        InitError,
    }

    impl SimulationModel for BooleanMaskSimulation {
        type Config = BooleanMaskConfig;
        type State = BooleanMaskState;
        type Error = SimulationError;

        fn initialize(config: Self::Config) -> Result<Self, Self::Error> {
            let state = BooleanMaskState {
                grid: vec![false; config.width * config.height],
                width: config.width,
                height: config.height,
            };
            Ok(Self { config, state })
        }

        fn step(&mut self) -> Result<(), Self::Error> {
            // Flip all bits
            for cell in self.state.grid.iter_mut() {
                *cell = !*cell;
            }
            Ok(())
        }

        fn get_state(&self) -> Self::State {
            self.state.clone()
        }
    }

    #[test]
    fn test_boolean_mask_simulation() {
        let config = BooleanMaskConfig {
            width: 10,
            height: 10,
        };
        let mut sim = BooleanMaskSimulation::initialize(config).unwrap();

        // Initial state should be all false
        assert!(sim.get_state().grid.iter().all(|&x| !x));

        // Step
        sim.step().unwrap();

        // State should be all true
        assert!(sim.get_state().grid.iter().all(|&x| x));
    }
}
pub mod mesh;
pub mod path_utils;
pub mod traceability;
pub mod vfs;
pub mod ast_visitor;
