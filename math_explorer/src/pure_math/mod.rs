//! # Pure Mathematics
//!
//! This module serves as the foundational core of the library, implementing rigorous mathematical
//! structures and algorithms. It is organized into several key domains ranging from abstract
//! algebra to differential geometry.
//!
//! ## Modules
//!
//! ### Foundations
//! * [`algebra`] - Abstract algebraic structures (Rings, Fields) and generic traits.
//! * [`graph_theory`] - Graph algorithms including Dijkstra's shortest path.
//!
//! ### Analysis & Geometry
//! * [`analysis`] - Numerical analysis, including ODE/PDE solvers (Runge-Kutta, Euler).
//! * [`differential_geometry`] - Surface calculus, heat equations on manifolds, and curvature flows.
//!
//! ### Number Theory & Cryptography
//! * [`number_theory`] - Prime generation, partition functions, and Q-series.
//! * [`elliptic_curves`] - Modular forms, p-adic valuations, and divisibility theorems.
//! * [`algorithmic_information`] - Kolmogorov complexity and information bounds.
//!
//! ## Usage
//!
//! ```rust
//! use math_explorer::pure_math::number_theory;
//!
//! // Check for primality
//! let n = 104729;
//! if number_theory::is_prime(n) {
//!     println!("{} is a prime number!", n);
//! }
//!
//! // Generate primes up to 20
//! let primes = number_theory::primes_up_to(20);
//! assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
//! ```

pub mod algebra;
pub mod algorithmic_information;
pub mod elliptic_curves;
pub mod number_theory;
pub mod graph_theory;
pub mod differential_geometry;
pub mod analysis;
