use crate::state::StateData;

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterValue {
    Float(f64),
    Int(i64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub value: ParameterValue,
    pub min: Option<ParameterValue>,
    pub max: Option<ParameterValue>,
}

pub trait GenericSimulation: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    
    fn get_parameters(&self) -> Vec<Parameter>;
    fn set_parameter(&mut self, name: &str, value: ParameterValue);
    
    fn reset(&mut self);
    fn step(&mut self, dt: f64, input: Option<f64>);
    
    fn get_state(&self) -> StateData;
}

pub trait SimulationRegistry: Send + Sync {
    fn register(&mut self, sim: Box<dyn GenericSimulation>);
    fn get_simulations(&mut self) -> Vec<Box<dyn GenericSimulation>>;
}
