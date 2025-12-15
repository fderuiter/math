
/// Biochemical Kinetics (Enzymes)
///
/// This module implements enzyme kinetics using the Michaelis-Menten framework.
/// The core equation describes the rate of enzymatic reactions by relating reaction rate $v$
/// to substrate concentration $[S]$.
///
/// $$ v = \frac{V_{max}[S]}{K_m + [S]} $$
///
/// where:
/// - $V_{max}$ is the maximum rate achieved by the system, at maximum (saturating) substrate concentrations.
/// - $K_m$ is the Michaelis constant, representing the substrate concentration at which the reaction rate is half of $V_{max}$.
pub mod kinetics {
    /// Represents an enzymatic reaction with defined kinetic parameters.
    pub struct EnzymeReaction {
        /// Maximum reaction rate ($V_{max}$).
        pub v_max: f64,
        /// Michaelis constant ($K_m$), substrate concentration at half $V_{max}$.
        pub k_m: f64,
    }

    impl EnzymeReaction {
        /// Creates a new EnzymeReaction with given parameters.
        pub fn new(v_max: f64, k_m: f64) -> Result<Self, String> {
            if v_max < 0.0 || k_m < 0.0 {
                return Err("Parameters V_max and K_m must be non-negative.".to_string());
            }
            Ok(Self { v_max, k_m })
        }

        /// Calculates the reaction velocity for a given substrate concentration $[S]$.
        ///
        /// Formula: $v = V_{max} \frac{[S]}{K_m + [S]}$
        ///
        /// # Arguments
        /// * `substrate_conc` - The concentration of the substrate ($[S]$).
        ///
        /// # Returns
        /// The reaction velocity. Returns an error if concentration is negative.
        pub fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String> {
            if substrate_conc < 0.0 {
                return Err("Substrate concentration cannot be negative.".to_string());
            }
            // Use 0.0 to handle potential division by zero if Km=0 and S=0, though physically Km > 0 usually.
            // If Km + S is 0, the rate is undefined/0.
            let denominator = self.k_m + substrate_conc;
            if denominator == 0.0 {
                return Ok(0.0);
            }

            Ok(self.v_max * substrate_conc / denominator)
        }
    }
}

/// Neuroscience (Hodgkin-Huxley)
///
/// This module implements the Hodgkin-Huxley model for a neuron's action potential.
/// The model describes how action potentials in neurons are initiated and propagated.
/// It is a set of nonlinear differential equations that approximates the electrical characteristics
/// of excitable cells such as neurons and cardiac myocytes.
///
/// The current through the membrane is given by:
/// $$ I = C_m \frac{dV}{dt} + I_{ion} $$
/// where $I_{ion}$ includes Sodium ($Na^+$), Potassium ($K^+$), and Leak ($L$) currents.
pub mod neuroscience {
    /// Represents the state of a Hodgkin-Huxley neuron.
    pub struct HodgkinHuxleyNeuron {
        /// Membrane potential (mV)
        pub v: f64,
        /// Gating variable for Potassium channel activation
        pub n: f64,
        /// Gating variable for Sodium channel activation
        pub m: f64,
        /// Gating variable for Sodium channel inactivation
        pub h: f64,

        /// Resting potential used for relative calculations (mV).
        pub v_rest: f64,
    }

    impl HodgkinHuxleyNeuron {
        pub fn new(v_initial: f64) -> Self {
            // Initialize gating variables to equilibrium at v_initial
            // For simplicity, we can start them at some standard values or calculate steady state.
            // Let's start with standard resting values approx.
            let v_rest = -65.0;
            Self {
                v: v_initial,
                n: 0.32,
                m: 0.05,
                h: 0.6,
                v_rest,
            }
        }

        fn alpha_n(v: f64, v_rest: f64) -> f64 {
            let x = 10.0 - (v - v_rest);
            if x.abs() < 1e-9 {
                0.1 // Limit as x -> 0
            } else {
                0.01 * x / ((0.1 * x).exp() - 1.0)
            }
        }

        fn beta_n(v: f64, v_rest: f64) -> f64 {
            let dv = v - v_rest;
            0.125 * (-dv / 80.0).exp()
        }

        fn alpha_m(v: f64, v_rest: f64) -> f64 {
            let dv = v - v_rest;
            let x = 25.0 - dv;
            if x.abs() < 1e-9 {
                1.0
            } else {
                0.1 * x / ((0.1 * x).exp() - 1.0)
            }
        }

        fn beta_m(v: f64, v_rest: f64) -> f64 {
            let dv = v - v_rest;
            4.0 * (-dv / 18.0).exp()
        }

        fn alpha_h(v: f64, v_rest: f64) -> f64 {
            let dv = v - v_rest;
            0.07 * (-dv / 20.0).exp()
        }

        fn beta_h(v: f64, v_rest: f64) -> f64 {
            let dv = v - v_rest;
            1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
        }

        /// Updates the neuron state by a time step `dt` with external current `i_ext`.
        /// Uses Euler integration for simplicity as requested/implied.
        pub fn update(&mut self, dt: f64, i_ext: f64) {
            // Constants
            let g_na = 120.0;
            let e_na = self.v_rest + 115.0; // Standard offset from rest
            let g_k = 36.0;
            let e_k = self.v_rest - 12.0;
            let g_l = 0.3;
            let e_l = self.v_rest + 10.6;

            // Calculate currents
            // I_tot equation from prompt: I_ext - g_Na m^3 h (V - E_Na) - g_K n^4 (V - E_K) - g_L (V - E_L)
            // Note: Standard HH usually has C_m * dV/dt = I_ext - I_ionic
            // Assuming C_m = 1.0 uF/cm^2

            let i_na = g_na * self.m.powi(3) * self.h * (self.v - e_na);
            let i_k = g_k * self.n.powi(4) * (self.v - e_k);
            let i_l = g_l * (self.v - e_l);

            let dv_dt = i_ext - i_na - i_k - i_l; // Assuming C_m = 1

            self.v += dv_dt * dt;

            // Update gating variables
            // dx/dt = alpha_x * (1 - x) - beta_x * x
            let update_gate = |x: f64, alpha: f64, beta: f64| -> f64 {
                let dx_dt = alpha * (1.0 - x) - beta * x;
                x + dx_dt * dt
            };

            self.n = update_gate(self.n, Self::alpha_n(self.v, self.v_rest), Self::beta_n(self.v, self.v_rest));
            self.m = update_gate(self.m, Self::alpha_m(self.v, self.v_rest), Self::beta_m(self.v, self.v_rest));
            self.h = update_gate(self.h, Self::alpha_h(self.v, self.v_rest), Self::beta_h(self.v, self.v_rest));
        }
    }
}

/// Morphogenesis (Turing Patterns)
///
/// This module implements a Reaction-Diffusion system capable of generating Turing patterns.
/// It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
///
/// The general equation is:
/// $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$
pub mod morphogenesis {
    /// Represents a 1D Reaction-Diffusion system.
    pub struct TuringSystem {
        /// Activator concentrations
        pub u: Vec<f64>,
        /// Inhibitor concentrations
        pub v: Vec<f64>,
        /// Diffusion coefficient for u
        pub d_u: f64,
        /// Diffusion coefficient for v
        pub d_v: f64,
        /// Grid spacing
        pub dx: f64,
    }

    impl TuringSystem {
        pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
            Self {
                u: vec![0.0; size],
                v: vec![0.0; size],
                d_u,
                d_v,
                dx,
            }
        }

        /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
        /// Using Gierer-Meinhardt-like kinetics as suggested:
        /// f(u,v) = a - u + u^2 v
        pub fn step(&mut self, dt: f64) {
            let n = self.u.len();
            let mut new_u = self.u.clone();
            let mut new_v = self.v.clone();

            let a = 0.01; // Feed rate / constant
            let b = 0.05; // Another constant
            // Using Schnakenberg-like kinetics for demonstration if not strictly specified beyond "f(u,v) = ..."

            for i in 0..n {
                // Laplacian with periodic boundary or zero-flux?
                // Zero flux (Neumann) is safer for 1D patterns usually, or periodic.
                // Using simple indices with clamping for Neumann-ish or just simple handling.
                let u_curr = self.u[i];
                let v_curr = self.v[i];

                let idx_prev = if i == 0 { 0 } else { i - 1 }; // Zero flux approx (u_-1 = u_0) -> deriv is 0
                let idx_next = if i == n - 1 { n - 1 } else { i + 1 };

                // Laplacian: (u_{i+1} - 2u_i + u_{i-1}) / dx^2
                // If i=0, u_{i-1} is u_0 -> (u_1 - 2u_0 + u_0) = u_1 - u_0.
                // This corresponds to forward difference at boundary, effectively zero flux if we consider ghost points.
                // Standard 3-point stencil.
                let lap_u = (self.u[idx_next] - 2.0 * u_curr + self.u[idx_prev]) / (self.dx * self.dx);
                let lap_v = (self.v[idx_next] - 2.0 * v_curr + self.v[idx_prev]) / (self.dx * self.dx);

                // Reaction terms
                // u_t = ... + a - u + u^2 v
                // v_t = ... + b - u^2 v (Schnakenberg)
                let reaction_u = a - u_curr + u_curr.powi(2) * v_curr;
                let reaction_v = b - u_curr.powi(2) * v_curr;

                new_u[i] = u_curr + dt * (self.d_u * lap_u + reaction_u);
                new_v[i] = v_curr + dt * (self.d_v * lap_v + reaction_v);
            }

            self.u = new_u;
            self.v = new_v;
        }
    }
}

/// Evolutionary Dynamics
///
/// This module implements Evolutionary Game Theory models, specifically the Hawk-Dove game.
/// It uses the Replicator Equation to model the evolution of strategy frequencies in a population.
///
/// The change in frequency of a strategy $i$ is given by:
/// $$ \frac{dp_i}{dt} = p_i (f_i(\mathbf{p}) - \phi(\mathbf{p})) $$
/// where $f_i$ is the fitness of strategy $i$ and $\phi$ is the average population fitness.
pub mod evolution {
    /// Represents a population playing the Hawk-Dove game.
    pub struct HawkDovePopulation {
        /// Value of the resource
        pub v: f64,
        /// Cost of injury
        pub c: f64,
    }

    impl HawkDovePopulation {
        pub fn new(v: f64, c: f64) -> Self {
            Self { v, c }
        }

        /// Updates the frequency of the Hawk strategy using the Replicator Equation.
        ///
        /// # Arguments
        /// * `hawk_freq` - Current frequency of Hawks ($p_H$). (Dove freq is $1 - p_H$).
        /// * `dt` - Time step.
        ///
        /// # Returns
        /// The new frequency of Hawks.
        pub fn update_frequencies(&self, hawk_freq: f64, dt: f64) -> Result<f64, String> {
            if hawk_freq < 0.0 || hawk_freq > 1.0 {
                return Err("Frequency must be between 0.0 and 1.0".to_string());
            }

            let p_h = hawk_freq;
            let p_d = 1.0 - p_h;

            // Payoffs
            // E(H, H) = (V-C)/2
            // E(H, D) = V
            // E(D, H) = 0
            // E(D, D) = V/2
            let e_hh = (self.v - self.c) / 2.0;
            let e_hd = self.v;
            let e_dh = 0.0;
            let e_dd = self.v / 2.0;

            // Fitness
            let fit_h = p_h * e_hh + p_d * e_hd;
            let fit_d = p_h * e_dh + p_d * e_dd;

            // Average Fitness
            let phi = p_h * fit_h + p_d * fit_d;

            // Differential: dp_H/dt = p_H (Fitness_H - phi)
            let dp_h = p_h * (fit_h - phi);

            // Update
            let mut new_p_h = p_h + dp_h * dt;

            // Clamp to [0, 1] to handle numerical drift
            if new_p_h < 0.0 { new_p_h = 0.0; }
            if new_p_h > 1.0 { new_p_h = 1.0; }

            Ok(new_p_h)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinetics_saturation() {
        // Vmax = 100.0, Km = 5.0
        let enzyme = kinetics::EnzymeReaction::new(100.0, 5.0).unwrap();
        // Huge substrate concentration
        let s = 1e6;
        let v = enzyme.reaction_velocity(s).unwrap();

        // Should be close to Vmax
        assert!((v - 100.0).abs() < 0.1, "Velocity {} should be close to Vmax 100.0", v);
    }

    #[test]
    fn test_evolution_ess() {
        // Hawk-Dove: V=2, C=4.
        // ESS is p_H = V/C = 0.5.
        // If p_H > V/C, Hawks do worse than average, p_H decreases.
        // If p_H < V/C, Hawks do better, p_H increases.
        let pop = evolution::HawkDovePopulation::new(2.0, 4.0);
        let mut p_hawk = 0.1; // Start low
        let dt = 0.1;

        for _ in 0..1000 {
            p_hawk = pop.update_frequencies(p_hawk, dt).unwrap();
        }

        assert!((p_hawk - 0.5).abs() < 1e-2, "Hawk frequency {} should converge to ESS 0.5", p_hawk);
    }

    #[test]
    fn test_neuroscience_update() {
        let mut neuron = neuroscience::HodgkinHuxleyNeuron::new(-65.0);
        // Step without current, should stay roughly stable or settle
        neuron.update(0.01, 0.0);
        // Check valid values
        assert!(neuron.v.is_finite());
        assert!(neuron.n >= 0.0 && neuron.n <= 1.0);
    }

    #[test]
    fn test_morphogenesis_step() {
        let mut sys = morphogenesis::TuringSystem::new(10, 0.1, 0.5, 1.0);
        // Initialize with some values
        sys.u[5] = 1.0;
        sys.v[5] = 0.5;

        sys.step(0.1);

        // Diffusion should spread the values
        assert!(sys.u[4] > 0.0);
        assert!(sys.u[6] > 0.0);
    }
}
