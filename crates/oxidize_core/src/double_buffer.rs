use crate::{ModelState, SimulationModel};

pub trait DoubleBufferedState: ModelState {
    fn swap_buffers(&mut self);
}

pub trait DoubleBufferedSimulationModel: SimulationModel
where
    Self::State: DoubleBufferedState,
{
    fn step_buffered(&mut self) -> Result<(), Self::Error> {
        self.step()?;
        // Wait, SimulationModel doesn't expose mutable state easily.
        // It exposes get_state() which returns a clone.
        Ok(())
    }
}
