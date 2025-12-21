//! # 3D Gaussian Splatting
//!
//! A rasterization-based technique for real-time view synthesis of 3D scenes.
//!
//! Unlike NeRFs which use volumetric ray-marching (expensive), Gaussian Splatting
//! represents the scene as a collection of 3D anisotropic Gaussians that are projected
//! ("splatted") onto the 2D image plane and blended using standard alpha compositing.
//!
//! ## Mathematical Formulation
//!
//! ### 1. The 3D Gaussian
//! A 3D Gaussian $G(x)$ is defined by a mean $\mu$ and covariance $\Sigma$:
//!
//! $$ G(x) = e^{-\frac{1}{2} (x - \mu)^T \Sigma^{-1} (x - \mu)} $$
//!
//! To ensure $\Sigma$ is positive semi-definite, it is parameterized by a scaling matrix $S$
//! and rotation matrix $R$:
//! $$ \Sigma = R S S^T R^T $$
//!
//! ### 2. Projection
//! When projecting to 2D, the covariance matrix $\Sigma$ is transformed to $\Sigma'$ in
//! image space using the viewing transformation $W$ and the projective Jacobian $J$:
//!
//! $$ \Sigma' = J W \Sigma W^T J^T $$
//!
//! ### 3. Rendering
//! Pixel color $C$ is computed by sorting Gaussians by depth and applying alpha blending:
//!
//! $$ C = \sum_{i \in N} c_i \alpha_i \prod_{j=1}^{i-1} (1 - \alpha_j) $$
//!
//! ## Pipeline Architecture
//!
//! ```mermaid
//! graph TD
//!     subgraph Scene
//!     G3D[3D Gaussians]
//!     Params[Pos, Scale, Rot, Opacity, SH]
//!     G3D --- Params
//!     end
//!
//!     subgraph Pipeline
//!     Proj[Projection to 2D]
//!     Sort[Depth Sorting]
//!     Blend[Alpha Blending]
//!     end
//!
//!     subgraph Optimization
//!     Loss[Loss Calculation]
//!     Dens[Densification & Pruning]
//!     end
//!
//!     G3D --> Proj
//!     Proj -->|2D Gaussians| Sort
//!     Sort -->|Sorted List| Blend
//!     Blend -->|Image| Loss
//!     Loss -->|Gradients| Dens
//!     Dens -->|Update| G3D
//! ```
//!
//! ## Example
//!
//! ```rust
//! use math_explorer::ai::gaussian_splatting::{Gaussian3D, Scene, rendering, projection};
//! use nalgebra::{Point3, Vector3, UnitQuaternion, Matrix4};
//!
//! fn main() {
//!     // 1. Create a simple scene with one Gaussian
//!     let gaussian = Gaussian3D {
//!         mean: Point3::new(0.0, 0.0, 5.0),
//!         scale: Vector3::new(1.0, 1.0, 1.0),
//!         rotation: UnitQuaternion::identity(),
//!         opacity: 0.8,
//!         color: Vector3::new(1.0, 0.0, 0.0), // Red
//!     };
//!
//!     let scene = Scene { gaussians: vec![gaussian] };
//!
//!     // 2. Define View and Projection matrices (Identity for simplicity)
//!     let view = Matrix4::<f64>::identity();
//!     let proj = Matrix4::<f64>::identity();
//!
//!     // 3. Project (Conceptual - see projection module for signature)
//!     // let p = projection::project_gaussian(&scene.gaussians[0], &view, &proj, 800.0, 600.0);
//! }
//! ```

pub mod structs;
pub mod projection;
pub mod rendering;
pub mod optimization;

pub use structs::{Gaussian3D, Gaussian2D, Scene};
pub use optimization::{determine_density_action, DensityAction};
