use crate::{ModelState, SimulationModel};

#[allow(missing_docs)]
pub trait DoubleBufferedState: ModelState {
    #[allow(missing_docs)]
    fn swap_buffers(&mut self);
}

#[allow(missing_docs)]
pub trait DoubleBufferedSimulationModel: SimulationModel
where
    Self::State: DoubleBufferedState,
{
    #[allow(missing_docs)]
    fn step_buffered(&mut self) -> Result<(), Self::Error> {
        self.step()?;
        // Wait, SimulationModel doesn't expose mutable state easily.
        // It exposes get_state() which returns a clone.
        Ok(())
    }
}
