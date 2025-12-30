use super::types::Point3D;

/// A trait representing a 3D scalar field that can be sampled at discrete grid points.
///
/// This abstraction allows the Marching Cubes algorithm to operate on various data sources,
/// such as explicit voxel grids, procedural noise functions, or mathematical formulas,
/// without being coupled to a specific storage implementation.
pub trait ScalarField3D {
    /// Returns the scalar value at the given grid coordinates.
    fn value(&self, x: usize, y: usize, z: usize) -> f32;

    /// Returns the dimensions of the grid (width, height, depth).
    fn dimensions(&self) -> (usize, usize, usize);

    /// Converts grid coordinates to world coordinates.
    ///
    /// This is used to position the generated vertices in 3D space.
    fn grid_to_world(&self, x: usize, y: usize, z: usize) -> Point3D;
}
