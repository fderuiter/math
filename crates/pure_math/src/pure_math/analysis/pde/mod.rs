/// Classification of Second-Order Linear PDEs.
/// Based on $B^2 - 4AC$.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdeClassification {
    Elliptic,
    Parabolic,
    Hyperbolic,
    Undefined, // If A=B=C=0 or degenerate
}

/// Represents the coefficients of a general 2nd order linear PDE:
/// $A u_{xx} + B u_{xy} + C u_{yy} + D u_x + E u_y + F u = G$
pub struct SecondOrderLinearPde2D {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    // Lower order terms don't affect classification
}

impl SecondOrderLinearPde2D {
    #[verified_engine::verified]
    pub fn classify(&self) -> PdeClassification {
        let discriminant = self.b * self.b - 4.0 * self.a * self.c;
        if discriminant < -1e-9 {
            PdeClassification::Elliptic
        } else if discriminant > 1e-9 {
            PdeClassification::Hyperbolic
        } else {
            PdeClassification::Parabolic
        }
    }
}

/// Strategy trait for evaluating boundary conditions.
///
/// Adheres to the Strategy Pattern and Open/Closed Principle, allowing
/// users to implement custom boundary condition types (e.g., Robin) without
/// modifying the core enum.
pub trait BoundaryCondition {
    /// Evaluates the boundary condition at the given spatial or spatio-temporal coordinates.
    #[verified_engine::verified]
    fn evaluate_boundary(&self, x: f64, y: f64) -> f64;
}

/// Dirichlet boundary condition: specifies the value $u = f$ on the boundary.
pub struct DirichletBoundary {
    pub function: Box<dyn Fn(f64, f64) -> f64>,
}

impl BoundaryCondition for DirichletBoundary {
    #[verified_engine::verified]
    fn evaluate_boundary(&self, x: f64, y: f64) -> f64 {
        (self.function)(x, y)
    }
}

/// Neumann boundary condition: specifies the normal derivative $\frac{\partial u}{\partial n} = g$.
pub struct NeumannBoundary {
    pub function: Box<dyn Fn(f64, f64) -> f64>,
}

impl BoundaryCondition for NeumannBoundary {
    #[verified_engine::verified]
    fn evaluate_boundary(&self, x: f64, y: f64) -> f64 {
        (self.function)(x, y)
    }
}

pub mod fused_stepper;
pub mod greens;
pub mod heat;
pub mod laplace;
pub mod wave;

// [cite:graph_parameters_rust]
