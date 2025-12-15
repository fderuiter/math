//! Epidemiology module for modeling disease spread.
//!
//! This module covers:
//! 1. Deterministic Compartmental Models (SIR, SEIR).
//! 2. Analytical solutions (Final Size).
//! 3. Matrix Algebra for R0 (Next Generation Matrix).
//! 4. Network Epidemiology (Heterogeneity).
//! 5. Stochastic Dynamics (Extinction, Gillespie).
//!
//! # Mathematical Background
//!
//! The **Threshold Theorem** states that an epidemic occurs if and only if the basic reproduction
//! number $R_0 > 1$.
//!
//! $R_0$ is defined as the expected number of secondary infections produced by a single infected
//! individual in a completely susceptible population.

use crate::pure_math::analysis::ode::{OdeSystem, RungeKutta4, VecState};

/// Deterministic Compartmental Models.
pub mod compartmental {
    use super::*;

    /// SIR Model: Susceptible, Infectious, Recovered.
    ///
    /// Equations:
    /// $$dS/dt = -\beta S I / N$$
    /// $$dI/dt = \beta S I / N - \gamma I$$
    /// $$dR/dt = \gamma I$$
    #[derive(Debug, Clone)]
    pub struct SIRModel {
        pub s: f64,
        pub i: f64,
        pub r: f64,
        pub n: f64,
        pub beta: f64,
        pub gamma: f64,
    }

    impl SIRModel {
        pub fn new(n: f64, i0: f64, beta: f64, gamma: f64) -> Self {
            Self {
                s: n - i0,
                i: i0,
                r: 0.0,
                n,
                beta,
                gamma,
            }
        }

        /// Advances the state by dt using Runge-Kutta 4.
        pub fn step(&mut self, dt: f64) {
            // Convert to VecState for the generic solver
            let state = VecState(vec![self.s, self.i, self.r]);
            let new_state = RungeKutta4::step(self, 0.0, &state, dt);

            self.s = new_state.0[0];
            self.i = new_state.0[1];
            self.r = new_state.0[2];
        }
    }

    impl OdeSystem<VecState> for SIRModel {
        fn derivative(&self, _t: f64, state: &VecState) -> VecState {
            let s = state.0[0];
            let i = state.0[1];
            // let r = state.0[2];

            let ds = -self.beta * s * i / self.n;
            let di = self.beta * s * i / self.n - self.gamma * i;
            let dr = self.gamma * i;

            VecState(vec![ds, di, dr])
        }
    }

    /// SEIR Model: Susceptible, Exposed, Infectious, Recovered.
    ///
    /// Equations:
    /// $$dE/dt = \beta S I / N - \sigma E$$
    /// $$dI/dt = \sigma E - \gamma I$$
    #[derive(Debug, Clone)]
    pub struct SEIRModel {
        pub s: f64,
        pub e: f64,
        pub i: f64,
        pub r: f64,
        pub n: f64,
        pub beta: f64,
        pub sigma: f64,
        pub gamma: f64,
    }

    impl SEIRModel {
        pub fn new(n: f64, i0: f64, beta: f64, sigma: f64, gamma: f64) -> Self {
            Self {
                s: n - i0,
                e: 0.0,
                i: i0,
                r: 0.0,
                n,
                beta,
                sigma,
                gamma,
            }
        }

        pub fn step(&mut self, dt: f64) {
            let state = VecState(vec![self.s, self.e, self.i, self.r]);
            let new_state = RungeKutta4::step(self, 0.0, &state, dt);

            self.s = new_state.0[0];
            self.e = new_state.0[1];
            self.i = new_state.0[2];
            self.r = new_state.0[3];
        }
    }

    impl OdeSystem<VecState> for SEIRModel {
        fn derivative(&self, _t: f64, state: &VecState) -> VecState {
            let s = state.0[0];
            let e = state.0[1];
            let i = state.0[2];
            // let r = state.0[3];

            let new_exposed = self.beta * s * i / self.n;
            let ds = -new_exposed;
            let de = new_exposed - self.sigma * e;
            let di = self.sigma * e - self.gamma * i;
            let dr = self.gamma * i;

            VecState(vec![ds, de, di, dr])
        }
    }

    pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
        if gamma == 0.0 {
            f64::INFINITY
        } else {
            beta / gamma
        }
    }
}

/// Analytical solutions for epidemiology.
pub mod analytics {
    /// Solves the final size equation for S_inf using Newton-Raphson.
    ///
    /// Equation: $\ln(S_0 / S_\infty) = R_0 (1 - S_\infty / N)$
    /// Rearranged for root finding: $f(x) = \ln(S_0 / x) - R_0 (1 - x / N) = 0$
    pub fn calculate_final_size(r0: f64, s0: f64, n: f64) -> Result<f64, String> {
        if r0 <= 0.0 {
            return Err("R0 must be positive".to_string());
        }

        // f(x) = ln(S0) - ln(x) - R0 + R0*x/N
        // f'(x) = -1/x + R0/N

        // Initial guess strategy:
        // If R0 > 1, the final size S_inf is approximately S0 * exp(-R0).
        // If R0 <= 1, the epidemic doesn't take off, S_inf ~ S0.
        let mut x = if r0 > 1.0 {
            s0 * (-r0).exp()
        } else {
            s0
        };

        // Ensure x is within reasonable bounds
        if x < 1e-5 { x = 1e-5; }
        if x > n { x = n - 1e-5; }

        let tolerance = 1e-7;
        let max_iter = 100;

        for _ in 0..max_iter {
            let fx = s0.ln() - x.ln() - r0 * (1.0 - x / n);
            let dfx = -1.0 / x + r0 / n;

            if dfx.abs() < 1e-10 {
                return Err("Derivative too close to zero".to_string());
            }

            let next_x = x - fx / dfx;

            if (next_x - x).abs() < tolerance {
                return Ok(next_x);
            }

            // Safety check to keep x within bounds
            if next_x <= 0.0 {
                 x /= 2.0; // Backtrack towards 0
            } else if next_x > n {
                 x = (x + n) / 2.0; // Backtrack towards N
            } else {
                x = next_x;
            }
        }

        Err("Newton-Raphson failed to converge".to_string())
    }
}

/// Matrix Algebra for Next Generation Matrix methods.
pub mod matrix_dynamics {
    use nalgebra::DMatrix;

    /// Calculates the Spectral Radius (R0) from Transmission (F) and Transition (V) matrices.
    ///
    /// $K = F \cdot V^{-1}$
    pub fn calculate_r0_matrix(f_mat: &DMatrix<f64>, v_mat: &DMatrix<f64>) -> Result<f64, String> {
        let v_inv = v_mat.clone().try_inverse().ok_or("Matrix V is singular")?;
        let k = f_mat * v_inv;

        let eigenvalues = k.complex_eigenvalues();

        let spectral_radius = eigenvalues.iter()
            .map(|c| c.norm())
            .fold(0.0, f64::max);

        Ok(spectral_radius)
    }
}

/// Network Epidemiology.
pub mod networks {
    /// Calculates R0 for a heterogeneous network.
    ///
    /// $R_0 = \frac{\beta}{\gamma} \frac{\langle k^2 \rangle - \langle k \rangle}{\langle k \rangle}$
    pub fn heterogeneous_r0(beta: f64, gamma: f64, mean_degree: f64, degree_variance: f64) -> f64 {
        if mean_degree == 0.0 || gamma == 0.0 {
            return 0.0;
        }

        // Var(k) = E[k^2] - (E[k])^2
        // E[k^2] = Var(k) + (E[k])^2

        let mean_k_sq = degree_variance + mean_degree.powi(2);
        let factor = (mean_k_sq - mean_degree) / mean_degree;

        (beta / gamma) * factor
    }
}

/// Stochastic Dynamics.
pub mod stochastic {
    use rand::Rng;

    /// Calculates the probability of extinction given R0 and initial cases.
    ///
    /// Based on Branching Process theory: $P_{ext} = (1/R_0)^{I_0}$ if $R_0 > 1$.
    pub fn probability_of_extinction(r0: f64, initial_cases: f64) -> f64 {
        if r0 <= 1.0 {
            1.0
        } else {
            (1.0 / r0).powf(initial_cases)
        }
    }

    /// Calculates time to next event for SIR system (Gillespie).
    ///
    /// $\tau = - \ln(U) / (\text{rate}_{infect} + \text{rate}_{recover})$
    pub fn gillespie_step_time(rate_infect: f64, rate_recover: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let u: f64 = rng.r#gen(); // Uniform (0, 1)

        // Avoid log(0)
        let u = if u == 0.0 { 1e-10 } else { u };

        let total_rate = rate_infect + rate_recover;
        if total_rate == 0.0 {
            return f64::INFINITY;
        }

        -u.ln() / total_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_threshold_theorem() {
        let n = 1000.0;
        let i0 = 10.0;
        // R0 = beta / gamma = 0.5 / 1.0 = 0.5 < 1
        let mut model = compartmental::SIRModel::new(n, i0, 0.5, 1.0);

        let initial_i = model.i;
        model.step(0.1);

        assert!(model.i < initial_i, "Infected should decrease when R0 < 1");
    }

    #[test]
    fn test_final_size_high_r0() {
        // R0 = 5.0
        let r0 = 5.0;
        let s0 = 999.0;
        let n = 1000.0;

        let s_inf = analytics::calculate_final_size(r0, s0, n).expect("Solver failed");

        // For R0=5, herd immunity threshold is 1 - 1/5 = 0.8 => 80% infected.
        // So S_inf should be small (less than 20% of N).
        assert!(s_inf < n * 0.2, "S_inf should be small for high R0");
        assert!(s_inf > 0.0);
    }

    #[test]
    fn test_matrix_r0_scalar_equivalence() {
        // 1x1 Matrix case should match scalar calculation
        let beta = 2.0;
        let gamma = 1.0;

        let f = DMatrix::from_vec(1, 1, vec![beta]);
        let v = DMatrix::from_vec(1, 1, vec![gamma]);

        let r0_matrix = matrix_dynamics::calculate_r0_matrix(&f, &v).unwrap();
        let r0_scalar = compartmental::basic_reproduction_number(beta, gamma);

        assert!((r0_matrix - r0_scalar).abs() < 1e-6);
    }

    #[test]
    fn test_heterogeneous_r0() {
        let beta = 0.5;
        let gamma = 0.1;
        // Homogeneous network: Variance = 0. Factor = (k^2 - k)/k = (k^2 - k)/k = k - 1?
        // Wait, if Var=0, then <k^2> = <k>^2.
        // Factor = (<k>^2 - <k>)/<k> = <k> - 1.
        // Standard formula usually assumes contact rate is proportional to k.

        // Using provided formula:
        let mean_k = 4.0;
        let var_k = 0.0;
        let r0 = networks::heterogeneous_r0(beta, gamma, mean_k, var_k);

        // R0 = (beta/gamma) * (16 - 4)/4 = 5 * 3 = 15.
        assert!((r0 - 15.0).abs() < 1e-6);
    }

    #[test]
    fn test_extinction_probability() {
        let r0 = 0.5;
        let i0 = 10.0;
        assert_eq!(stochastic::probability_of_extinction(r0, i0), 1.0);

        let r0_high = 2.0;
        let i0_one = 1.0;
        // P = 1/2
        assert!((stochastic::probability_of_extinction(r0_high, i0_one) - 0.5).abs() < 1e-6);
    }
}
