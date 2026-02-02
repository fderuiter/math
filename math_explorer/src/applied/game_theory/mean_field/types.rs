use nalgebra::DMatrix;

/// Represents a 1D Mean Field Game (MFG) configuration.
///
/// # Field Descriptions
/// - `viscosity` ($\nu$): Diffusion parameter representing noise/randomness in agent motion.
/// - `time_horizon` ($T$): Duration of the game.
/// - `dt`, `dx`: Discretization steps.
#[derive(Clone, Debug)]
pub struct MFGConfig {
    pub viscosity: f64,     // nu
    pub time_horizon: f64,  // T
    pub time_steps: usize,  // Nt
    pub grid_points: usize, // Nx
    pub dt: f64,
    pub dx: f64,
    pub space_min: f64,
    pub space_max: f64,
}

impl MFGConfig {
    pub fn new(
        viscosity: f64,
        time_horizon: f64,
        grid_points: usize,
        time_steps: usize,
        space_min: f64,
        space_max: f64,
    ) -> Self {
        let dt = time_horizon / (time_steps as f64);
        let dx = (space_max - space_min) / ((grid_points - 1) as f64);
        Self {
            viscosity,
            time_horizon,
            time_steps,
            grid_points,
            dt,
            dx,
            space_min,
            space_max,
        }
    }
}

/// Result of a Mean Field Game simulation.
///
/// Encapsulates the solution fields for value function and population distribution.
#[derive(Debug, Clone)]
pub struct MeanFieldSolution {
    /// The value function $u(x, t)$ representing the optimal cost-to-go.
    pub value_function: DMatrix<f64>,
    /// The population distribution $m(x, t)$.
    pub distribution: DMatrix<f64>,
}
