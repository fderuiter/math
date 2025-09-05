//! # Mathematics of the Simulation Study
//!
//! This module implements the formulas used in the paper's simulation study.
//! It includes joint survival functions, the win ratio parameter, and derived
//! marginal and conditional survival functions and their PDFs.

/// Parameters for the simulation study.
#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    /// Lambda1 parameter.
    pub lambda1: f64,
    /// Lambda2 parameter.
    pub lambda2: f64,
    /// Alpha parameter (correlation).
    pub alpha: f64,
    /// Theta parameter (treatment effect).
    pub theta: f64,
}

impl SimulationParams {
    /// Creates a new `SimulationParams`.
    pub fn new(lambda1: f64, lambda2: f64, alpha: f64, theta: f64) -> Self {
        Self { lambda1, lambda2, alpha, theta }
    }
}

// --- Joint Survival Functions ---

/// Joint survival function for the control group, S0(t,x).
pub fn joint_survival_function_control(t: f64, x: f64, params: &SimulationParams) -> f64 {
    (-(params.lambda1 * t + params.lambda2 * x).powf(params.alpha)).exp()
}

/// Joint survival function for the treatment group, S1(t,x).
pub fn joint_survival_function_treatment(t: f64, x: f64, params: &SimulationParams) -> f64 {
    (-(params.theta * params.lambda1 * t + params.theta * params.lambda2 * x).powf(params.alpha)).exp()
}

// --- Win Ratio Parameter ---

/// Win ratio parameter in the absence of censoring, PR_W.
pub fn win_ratio_parameter(params: &SimulationParams) -> f64 {
    1.0 / params.theta.powf(params.alpha)
}

// --- Derived Functions for T (Fatal Event) ---

/// Marginal survival function for T in the control group, S0(t).
pub fn marginal_survival_t_control(t: f64, params: &SimulationParams) -> f64 {
    joint_survival_function_control(t, 0.0, params)
}

/// Marginal survival function for T in the treatment group, S1(t).
pub fn marginal_survival_t_treatment(t: f64, params: &SimulationParams) -> f64 {
    joint_survival_function_treatment(t, 0.0, params)
}

/// PDF for T in the control group, f0(t).
pub fn pdf_t_control(t: f64, params: &SimulationParams) -> f64 {
    if t < 0.0 { return 0.0; }
    if t == 0.0 { return 0.0; } // Handle t=0 case for t.powf(alpha - 1.0)
    let l1_alpha = params.lambda1.powf(params.alpha);
    params.alpha * l1_alpha * t.powf(params.alpha - 1.0) * (-(l1_alpha * t.powf(params.alpha))).exp()
}

/// PDF for T in the treatment group, f1(t).
pub fn pdf_t_treatment(t: f64, params: &SimulationParams) -> f64 {
    if t < 0.0 { return 0.0; }
    if t == 0.0 { return 0.0; }
    let th_l1_alpha = (params.theta * params.lambda1).powf(params.alpha);
    params.alpha * th_l1_alpha * t.powf(params.alpha - 1.0) * (-(th_l1_alpha * t.powf(params.alpha))).exp()
}


// --- Derived Functions for X (Non-Fatal Event) given T > c ---

/// Conditional survival function for X in the control group, G0(x|c).
pub fn conditional_survival_x_given_t_control(x: f64, c: f64, params: &SimulationParams) -> f64 {
    let s0_c_x = joint_survival_function_control(c, x, params);
    let s0_c_0 = marginal_survival_t_control(c, params);
    if s0_c_0 == 0.0 { 0.0 } else { s0_c_x / s0_c_0 }
}

/// Conditional survival function for X in the treatment group, G1(x|c).
pub fn conditional_survival_x_given_t_treatment(x: f64, c: f64, params: &SimulationParams) -> f64 {
    let s1_c_x = joint_survival_function_treatment(c, x, params);
    let s1_c_0 = marginal_survival_t_treatment(c, params);
    if s1_c_0 == 0.0 { 0.0 } else { s1_c_x / s1_c_0 }
}

/// Conditional PDF for X in the control group, f0(x|c).
pub fn pdf_x_given_t_control(x: f64, c: f64, params: &SimulationParams) -> f64 {
    if x < 0.0 { return 0.0; }
    let term = params.lambda1 * c + params.lambda2 * x;
    if term < 0.0 { return 0.0; }
    let g0 = conditional_survival_x_given_t_control(x, c, params);
    params.alpha * params.lambda2 * term.powf(params.alpha - 1.0) * g0
}

/// Conditional PDF for X in the treatment group, f1(x|c).
pub fn pdf_x_given_t_treatment(x: f64, c: f64, params: &SimulationParams) -> f64 {
    if x < 0.0 { return 0.0; }
    let term = params.theta * params.lambda1 * c + params.theta * params.lambda2 * x;
    if term < 0.0 { return 0.0; }
    let g1 = conditional_survival_x_given_t_treatment(x, c, params);
    params.alpha * params.theta * params.lambda2 * term.powf(params.alpha - 1.0) * g1
}
