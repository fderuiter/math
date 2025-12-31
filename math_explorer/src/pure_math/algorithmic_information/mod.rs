//! # Algorithmic Information Theory
//!
//! This module explores concepts from **Algorithmic Information Theory (AIT)**, focusing on
//! Kolmogorov complexity and its applications to geometric bounds.
//!
//! It implements algorithms discussed in the paper *"Algorithmic Information Bounds for Distances and Orthogonal Projections"*,
//! enabling the approximation of information content in numerical structures.
//!
//! ## Key Concepts
//!
//! * **Kolmogorov Complexity ($K(x)$):** The length of the shortest program that outputs $x$.
//!   Since $K(x)$ is uncomputable, we provide computable approximations.
//! * **Prefix Complexity:** A variant of $K(x)$ where the set of valid programs forms a prefix-free code.
//! * **Information Bounds:** Theoretical limits on the information shared between geometric objects (e.g., points and lines).
//!
//! ## Dependencies
//!
//! This module relies on the [`rug`](https://crates.io/crates/rug) crate for arbitrary-precision arithmetic,
//! which is essential for handling the large numbers and precise rational approximations required by AIT.
//!
//! ## Usage
//!
//! ```rust
//! use math_explorer::pure_math::algorithmic_information::kolmogorov;
//! use rug::Integer;
//!
//! // Approximate the Kolmogorov complexity of a large integer
//! let n = Integer::from(123456789);
//! let complexity = kolmogorov::prefix_kolmogorov_approx(&n);
//!
//! println!("Approximate K({}) ≈ {:.2} bits", n, complexity);
//! ```

pub mod combinatorics;
pub mod geometry;
pub mod kolmogorov;
