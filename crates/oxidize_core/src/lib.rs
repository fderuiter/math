pub trait ModelConfig: Clone + serde::Serialize + serde::Deserialize<'static> {}

pub trait ModelState: Clone {}

pub trait SimulationModel {
    type Config;
    type Error;
    type State;

    fn initialize(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn step(&mut self) -> Result<(), Self::Error>;

    fn get_state(&self) -> Self::State;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
