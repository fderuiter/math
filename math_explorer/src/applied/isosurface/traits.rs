use super::types::Point3D;

/// A trait representing a 3D scalar field.
///
/// This abstraction allows the Marching Cubes algorithm to operate on:
/// 1. Discrete voxel grids (e.g., medical imaging data).
/// 2. Procedural functions (e.g., Perlin noise, implicit surfaces).
///
/// By implementing this trait, you can generate meshes from mathematical formulas
/// without allocating a massive 3D array in memory.
pub trait ScalarField3D {
    /// Returns the width (number of voxels along X).
    fn width(&self) -> usize;

    /// Returns the height (number of voxels along Y).
    fn height(&self) -> usize;

    /// Returns the depth (number of voxels along Z).
    fn depth(&self) -> usize;

    /// Returns the scalar value at the given grid coordinates.
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1).
    /// * `y` - Y coordinate (0 to height-1).
    /// * `z` - Z coordinate (0 to depth-1).
    fn get(&self, x: usize, y: usize, z: usize) -> f32;

    /// Returns the size of a single voxel in physical units.
    fn voxel_size(&self) -> Point3D;

    /// Returns the physical origin of the grid (coordinate of 0,0,0).
    fn origin(&self) -> Point3D;
}
