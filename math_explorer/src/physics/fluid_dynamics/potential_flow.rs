//! 2D Potential Flow Elements (Inviscid, Irrotational, Incompressible).
//!
//! This module implements standard elementary flows that can be superimposed to model complex flow fields.
//!
//! # Theory
//! Potential flow assumes the fluid is inviscid, incompressible, and irrotational.
//! The flow can be described by a velocity potential $\phi$ and a stream function $\psi$.
//!
//! * Velocity: $\mathbf{V} = \nabla \phi$
//! * Cauchy-Riemann: $u = \frac{\partial \phi}{\partial x} = \frac{\partial \psi}{\partial y}$, $v = \frac{\partial \phi}{\partial y} = -\frac{\partial \psi}{\partial x}$

use nalgebra::Vector2;
use std::f64::consts::PI;

/// A fundamental element of a 2D potential flow field.
pub trait FlowElement: Send + Sync {
    /// Velocity vector (u, v) at point (x, y).
    fn velocity(&self, x: f64, y: f64) -> Vector2<f64>;

    /// Stream function value psi at point (x, y).
    fn stream_function(&self, x: f64, y: f64) -> f64;

    /// Velocity potential phi at point (x, y).
    fn potential(&self, x: f64, y: f64) -> f64;
}

/// Uniform flow with constant velocity U at angle alpha.
#[derive(Debug, Clone, Copy)]
pub struct UniformFlow {
    pub velocity: f64,
    pub angle: f64, // radians
}

impl UniformFlow {
    pub fn new(velocity: f64, angle_degrees: f64) -> Self {
        Self {
            velocity,
            angle: angle_degrees.to_radians(),
        }
    }
}

impl FlowElement for UniformFlow {
    fn velocity(&self, _x: f64, _y: f64) -> Vector2<f64> {
        Vector2::new(
            self.velocity * self.angle.cos(),
            self.velocity * self.angle.sin(),
        )
    }

    fn stream_function(&self, x: f64, y: f64) -> f64 {
        self.velocity * (y * self.angle.cos() - x * self.angle.sin())
    }

    fn potential(&self, x: f64, y: f64) -> f64 {
        self.velocity * (x * self.angle.cos() + y * self.angle.sin())
    }
}

/// A Source (strength > 0) or Sink (strength < 0) at a specific location.
///
/// * Strength $Q$: Volumetric flow rate per unit depth.
#[derive(Debug, Clone, Copy)]
pub struct Source {
    pub strength: f64,
    pub x: f64,
    pub y: f64,
}

impl Source {
    pub fn new(strength: f64, x: f64, y: f64) -> Self {
        Self { strength, x, y }
    }
}

impl FlowElement for Source {
    fn velocity(&self, x: f64, y: f64) -> Vector2<f64> {
        let dx = x - self.x;
        let dy = y - self.y;
        let r2 = dx * dx + dy * dy;
        if r2 < 1e-10 {
            return Vector2::zeros();
        }
        let u = (self.strength * dx) / (2.0 * PI * r2);
        let v = (self.strength * dy) / (2.0 * PI * r2);
        Vector2::new(u, v)
    }

    fn stream_function(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        (self.strength / (2.0 * PI)) * dy.atan2(dx)
    }

    fn potential(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        let r = (dx * dx + dy * dy).sqrt();
        (self.strength / (2.0 * PI)) * r.ln()
    }
}

/// A Free Vortex with circulation Gamma at a specific location.
///
/// * Strength $\Gamma$: Circulation (positive is counter-clockwise).
#[derive(Debug, Clone, Copy)]
pub struct Vortex {
    pub strength: f64,
    pub x: f64,
    pub y: f64,
}

impl Vortex {
    pub fn new(strength: f64, x: f64, y: f64) -> Self {
        Self { strength, x, y }
    }
}

impl FlowElement for Vortex {
    fn velocity(&self, x: f64, y: f64) -> Vector2<f64> {
        let dx = x - self.x;
        let dy = y - self.y;
        let r2 = dx * dx + dy * dy;
        if r2 < 1e-10 {
            return Vector2::zeros();
        }
        // u = -Gamma/(2*pi*r) * sin(theta) = -Gamma*y / (2*pi*r^2)
        // v = Gamma/(2*pi*r) * cos(theta) = Gamma*x / (2*pi*r^2)
        let u = -(self.strength * dy) / (2.0 * PI * r2);
        let v = (self.strength * dx) / (2.0 * PI * r2);
        Vector2::new(u, v)
    }

    fn stream_function(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        let r = (dx * dx + dy * dy).sqrt();
        if r < 1e-10 {
            return 0.0;
        }
        (self.strength / (2.0 * PI)) * r.ln()
    }

    fn potential(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        -(self.strength / (2.0 * PI)) * dy.atan2(dx)
    }
}

/// A Doublet (Source + Sink at infinitesimal distance) at a location.
///
/// Commonly used to simulate flow around a cylinder when combined with uniform flow.
#[derive(Debug, Clone, Copy)]
pub struct Doublet {
    pub strength: f64, // Kappa
    pub x: f64,
    pub y: f64,
}

impl Doublet {
    pub fn new(strength: f64, x: f64, y: f64) -> Self {
        Self { strength, x, y }
    }
}

impl FlowElement for Doublet {
    fn velocity(&self, x: f64, y: f64) -> Vector2<f64> {
        let dx = x - self.x;
        let dy = y - self.y;
        let r2 = dx * dx + dy * dy;
        let r4 = r2 * r2;
        if r2 < 1e-10 {
            return Vector2::zeros();
        }

        // For doublet aligned with X-axis:
        // u = -kappa * (x^2 - y^2) / (2*pi*r^4)
        // v = -kappa * (2xy) / (2*pi*r^4)
        // Note: Sign convention varies.
        // If phi = -kappa * x / (2*pi*r^2) (doublet pointing +x)
        // Then u = dphi/dx = -kappa/2pi * (1*r^2 - x*2x*x/r)/r^4 ??? No.
        // d(x/r^2)/dx = (1*r^2 - x*2x)/r^4 = (x^2+y^2 - 2x^2)/r^4 = (y^2-x^2)/r^4
        // So u = -kappa/2pi * (y^2-x^2)/r^4 = kappa/2pi * (x^2-y^2)/r^4

        // Wait, standard result for cylinder radius R in stream U:
        // Doublet strength kappa = 2*pi*U*R^2

        let u_num = self.strength * (dx * dx - dy * dy);
        let v_num = self.strength * (2.0 * dx * dy);

        let u = -u_num / (2.0 * PI * r4);
        let v = -v_num / (2.0 * PI * r4);

        Vector2::new(u, v)
    }

    fn stream_function(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        let r2 = dx * dx + dy * dy;
        if r2 < 1e-10 {
            return 0.0;
        }
        // psi = - (kappa * y) / (2*pi*r^2)
        -(self.strength * dy) / (2.0 * PI * r2)
    }

    fn potential(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        let r2 = dx * dx + dy * dy;
        if r2 < 1e-10 {
            return 0.0;
        }
        // phi = - (kappa * x) / (2*pi*r^2)
        -(self.strength * dx) / (2.0 * PI * r2)
    }
}

/// A container for multiple flow elements.
#[derive(Default)]
pub struct PotentialFlowField {
    pub elements: Vec<Box<dyn FlowElement>>,
}

impl PotentialFlowField {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn add(&mut self, element: Box<dyn FlowElement>) {
        self.elements.push(element);
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

impl FlowElement for PotentialFlowField {
    fn velocity(&self, x: f64, y: f64) -> Vector2<f64> {
        let mut u_total = Vector2::zeros();
        for element in &self.elements {
            u_total += element.velocity(x, y);
        }
        u_total
    }

    fn stream_function(&self, x: f64, y: f64) -> f64 {
        let mut psi_total = 0.0;
        for element in &self.elements {
            psi_total += element.stream_function(x, y);
        }
        psi_total
    }

    fn potential(&self, x: f64, y: f64) -> f64 {
        let mut phi_total = 0.0;
        for element in &self.elements {
            phi_total += element.potential(x, y);
        }
        phi_total
    }
}
