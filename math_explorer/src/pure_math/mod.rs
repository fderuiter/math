//! # Pure Mathematics
//!
//! This module serves as the foundation for mathematical structures and algorithms
//! within the `math_explorer` crate. It encompasses a wide range of disciplines,
//! from abstract algebra and number theory to differential geometry and analysis.
//!
//! ## Submodules
//!
//! - [`algebra`]: Abstract and linear algebra concepts, including the [`algebra::Ring`] trait.
//! - [`algorithmic_information`]: Tools for exploring algorithmic information theory, including Kolmogorov complexity.
//! - [`analysis`]: Mathematical analysis, providing solvers for Ordinary Differential Equations (ODEs) and Partial Differential Equations (PDEs).
//! - [`differential_geometry`]: Surface calculus, operators, and geometric flows like Mean Curvature Flow.
//! - [`elliptic_curves`]: Algorithms related to elliptic curves and modular polynomials, including bounds on p-adic valuations.
//! - [`graph_theory`]: Graph algorithms (e.g., Dijkstra) and graph parameters (treewidth, degree).
//! - [`number_theory`]: Core number-theoretic functions, including primality testing, partitions, and q-series.
//!
//! ## Usage
//!
//! Most functionalities can be accessed by importing the specific submodule of interest.
//!
//! ```rust
//! use math_explorer::pure_math::number_theory::is_prime;
//!
//! assert!(is_prime(7));
//! assert!(!is_prime(10));
//! ```

pub mod algebra;
pub mod algorithmic_information;
pub mod elliptic_curves;
pub mod number_theory;
pub mod graph_theory;
pub mod differential_geometry;
pub mod analysis;
