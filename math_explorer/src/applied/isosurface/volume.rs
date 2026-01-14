use super::types::Point3D;

/// A trait representing a 3D scalar field or volume.
///
/// This allows isosurface extraction from various sources, such as dense voxel grids,
/// sparse arrays, or analytical functions (implicit surfaces).
pub trait Volume {
    /// Returns the dimensions of the volume (width, height, depth).
    fn dimensions(&self) -> (usize, usize, usize);

    /// Retrieves the scalar value at the given grid coordinates.
    ///
    /// # Arguments
    /// * `x` - X index.
    /// * `y` - Y index.
    /// * `z` - Z index.
    ///
    /// # Returns
    /// * `f32` - The scalar value (e.g., density, signed distance).
    fn get(&self, x: usize, y: usize, z: usize) -> f32;

    /// Returns the physical size of each voxel (dx, dy, dz).
    fn voxel_size(&self) -> Point3D;

    /// Returns the physical origin of the volume (0,0,0 coordinate).
    fn origin(&self) -> Point3D;
}
