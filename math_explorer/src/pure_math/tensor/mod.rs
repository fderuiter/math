//! # Tensor Calculus
//!
//! This module provides the mathematical machinery for performing calculus on curved manifolds.
//! It implements the core objects required for General Relativity and Differential Geometry.
//!
//! ## 🗺️ Concepts
//!
//! Calculus on curved surfaces requires more than just standard derivatives. We need tools that
//! account for how the coordinate system itself stretches and twists.
//!
//! ```mermaid
//! graph TD
//!     Metric[Metric Tensor g_ij] -->|Defines Distance| Chris[Christoffel Symbols Γ]
//!     Chris -->|Defines Parallel Transport| CovDeriv[Covariant Derivative ∇]
//!     CovDeriv -->|Defines Straight Lines| Geodesic[Geodesic Equation]
//!     CovDeriv -->|Measures Curvature| Riemann[Riemann Curvature Tensor]
//!
//!     style Metric fill:#f9f,stroke:#333,stroke-width:2px
//!     style CovDeriv fill:#bbf,stroke:#333
//! ```
//!
//! ### 1. The Metric Tensor ($g_{ij}$)
//! The "ruler" of the space. It defines the distance between two infinitesimally close points.
//! $$ ds^2 = g_{ij} dx^i dx^j $$
//!
//! ### 2. Covariant vs. Contravariant
//! - **Contravariant Vectors ($A^\mu$)**: Represent displacement or velocity (e.g., $dx^\mu$). Indices are **Upper**.
//! - **Covariant Vectors ($A_\mu$)**: Represent gradients or normals (e.g., $\frac{\partial \phi}{\partial x^\mu}$). Indices are **Lower**.
//!
//! The Metric Tensor allows us to switch between them (raising and lowering indices):
//! $$ A_\mu = g_{\mu\nu} A^\nu $$
//!
//! ### 3. Christoffel Symbols ($\Gamma^\lambda_{\mu\nu}$)
//! Correction terms that account for the curvature of the coordinate system. They appear when differentiating basis vectors.
//!
//! ## 🚀 Quick Start: Geometry of a Sphere
//!
//! Calculate the Christoffel symbols for a 2D Sphere of radius 1 using Polar Coordinates.
//!
//! $$ ds^2 = d\theta^2 + \sin^2\theta d\phi^2 $$
//!
//! ```rust
//! use math_explorer::pure_math::tensor::christoffel::christoffel_symbols;
//! use math_explorer::pure_math::tensor::metric::{Metric, RiemannianMetric};
//! use nalgebra::{DMatrix, DVector};
//!
//! fn main() {
//!     // 1. Define the Metric for a 2D Sphere (Radius = 1)
//!     // Coordinates: x0 = theta (polar), x1 = phi (azimuthal)
//!     // Metric Matrix: Diag(1, sin^2(theta))
//!     let sphere_metric = RiemannianMetric::new(|point: &DVector<f64>| {
//!         let theta = point[0];
//!         let sin_theta = theta.sin();
//!
//!         DMatrix::from_row_slice(2, 2, &[
//!             1.0, 0.0,
//!             0.0, sin_theta * sin_theta
//!         ])
//!     });
//!
//!     // 2. Choose a point: theta = pi/4 (45 degrees), phi = 0
//!     let point = DVector::from_vec(vec![std::f64::consts::FRAC_PI_4, 0.0]);
//!
//!     // 3. Compute Christoffel Symbols
//!     let gammas = christoffel_symbols(&sphere_metric, &point).unwrap();
//!
//!     // 4. Verify specific symbol: Gamma^theta_phi_phi = -sin(theta)cos(theta)
//!     // At pi/4, sin=cos=1/sqrt(2), so product is 0.5. Result should be -0.5.
//!     let gamma_theta_phi_phi = gammas[0][(1, 1)];
//!     println!("Gamma^theta_phi_phi: {:.4}", gamma_theta_phi_phi);
//!
//!     assert!((gamma_theta_phi_phi - (-0.5)).abs() < 1e-4);
//! }
//! ```

pub mod christoffel;
pub mod differentiation;
pub mod metric;
pub mod types;
