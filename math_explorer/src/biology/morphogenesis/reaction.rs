use crate::biology::reaction_diffusion::ReactionModel;

/// Defines the reaction kinetics for a 2-component reaction-diffusion system.
pub trait ReactionKinetics {
    /// Calculates the reaction rates for activator u and inhibitor v.
    ///
    /// # Arguments
    /// * `u` - Concentration of activator.
    /// * `v` - Concentration of inhibitor.
    ///
    /// # Returns
    /// A tuple `(du/dt, dv/dt)` representing the reaction terms.
    fn reaction(&self, u: f64, v: f64) -> (f64, f64);
}

/// Schnakenberg kinetics (often used for Turing patterns).
///
/// This model is famous for generating spot-like patterns (like leopard spots).
///
/// ## Equations
///
/// $$ \frac{du}{dt} = a - u + u^2 v $$
/// $$ \frac{dv}{dt} = b - u^2 v $$
///
/// Where:
/// - $a$: Production rate of the activator.
/// - $b$: Production rate of the inhibitor.
/// - $u^2 v$: Non-linear autocatalysis term (Activator requires Inhibitor to grow, but consumes it).
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    /// Production rate of activator ($a$).
    pub a: f64,
    /// Production rate of inhibitor ($b$).
    pub b: f64,
}

impl SchnakenbergKinetics {
    /// Creates a new Schnakenberg kinetics model.
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }
}

impl Default for SchnakenbergKinetics {
    fn default() -> Self {
        Self { a: 0.01, b: 0.05 }
    }
}

impl ReactionKinetics for SchnakenbergKinetics {
    fn reaction(&self, u: f64, v: f64) -> (f64, f64) {
        let uv_sq = u * u * v;
        let reaction_u = self.a - u + uv_sq;
        let reaction_v = self.b - uv_sq;
        (reaction_u, reaction_v)
    }
}

impl ReactionModel for SchnakenbergKinetics {
    fn reaction(&self, concentrations: &[f64], rates: &mut [f64]) {
        if concentrations.len() < 2 || rates.len() < 2 {
            return;
        }
        let u = concentrations[0];
        let v = concentrations[1];
        let (du, dv) = <Self as ReactionKinetics>::reaction(self, u, v);
        rates[0] = du;
        rates[1] = dv;
    }

    fn add_reaction_batch(&self, concentrations: &[Vec<f64>], rates: &mut [Vec<f64>]) {
        if concentrations.len() < 2 || rates.len() < 2 {
            return;
        }

        let u_vec = &concentrations[0];
        let v_vec = &concentrations[1];

        // Split mutable borrow to access both rate vectors simultaneously
        let (left, right) = rates.split_at_mut(1);
        let rates_u = &mut left[0];
        let rates_v = &mut right[0];

        let n = u_vec
            .len()
            .min(v_vec.len())
            .min(rates_u.len())
            .min(rates_v.len());

        // Vectorized loop: Access memory linearly, enabling prefetch and SIMD
        for i in 0..n {
            let (du, dv) = <Self as ReactionKinetics>::reaction(self, u_vec[i], v_vec[i]);
            rates_u[i] += du;
            rates_v[i] += dv;
        }
    }
}
