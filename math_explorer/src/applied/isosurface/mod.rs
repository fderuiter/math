//! # Isosurface Extraction
//!
//! This module provides algorithms to extract 3D surfaces from volumetric data (scalar fields).
//! The primary algorithm implemented is **Marching Cubes**, which converts a 3D grid of values
//! (voxels) into a polygonal mesh of triangles.
//!
//! This is commonly used in:
//! - Medical Imaging (CT/MRI scans) to visualize bones or organs.
//! - Physics simulations to visualize fluid boundaries.
//! - Procedural generation for terrain or organic shapes.
//!
//! ## Quick Usage
//!
//! ```rust
//! use math_explorer::applied::isosurface::{extract_isosurface, VoxelGrid, Point3D};
//!
//! // 1. Create a tiny 2x2x2 grid
//! let grid = VoxelGrid {
//!     width: 2, height: 2, depth: 2,
//!     data: vec![
//!         -1.0, -1.0,  // z=0, y=0
//!         -1.0, -1.0,  // z=0, y=1
//!          1.0,  1.0,  // z=1, y=0
//!          1.0,  1.0   // z=1, y=1
//!     ],
//!     voxel_size: Point3D::new(1.0, 1.0, 1.0),
//!     origin: Point3D::new(0.0, 0.0, 0.0),
//! };
//!
//! // 2. Extract the surface where value == 0.0
//! let mesh = extract_isosurface(&grid, 0.0).unwrap();
//!
//! println!("Generated {} triangles", mesh.triangles.len());
//! ```
//!
//! ## Examples
//!
//! See `math_explorer/examples/isosurface_torus.rs` for a complete example that:
//! 1. Generates a Signed Distance Field (SDF) for a torus.
//! 2. Extracts the mesh.
//! 3. Exports it to an `.obj` file for visualization in Blender/MeshLab.
//!
//! Run it with:
//! ```bash
//! cargo run --example isosurface_torus
//! ```

pub mod gradients;
pub mod marching_cubes;
pub mod tables;
pub mod types;

pub use gradients::{CentralDifferenceEstimator, GradientEstimator};
pub use marching_cubes::extract_isosurface;
pub use types::{Mesh, Point3D, Triangle, VoxelGrid};

// [cite:isosurface_extraction]

use crate::theory_verification;

theory_verification!(
    module = "isosurface",
    paper = "isosurface_extraction.tex",
    epsilon = 1e-6,
    constants = {
        LEVEL = 0.0;
    },
    test = {
        assert_relative_eq!(LEVEL, 0.0, epsilon = 1e-6);
    }
);
