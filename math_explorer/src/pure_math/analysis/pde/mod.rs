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

/// Boundary condition types.
pub enum BoundaryCondition {
    /// Specify value $u = f$
    Dirichlet(Box<dyn Fn(f64, f64) -> f64>),
    /// Specify derivative $\frac{\partial u}{\partial n} = g$
    Neumann(Box<dyn Fn(f64, f64) -> f64>),
    // Robin omitted for simplicity but follows $\alpha u + \beta u_n = h$
}

pub mod greens;
pub mod heat;
pub mod laplace;
pub mod wave;
