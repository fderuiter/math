use super::types::{Point3D, VoxelGrid};

/// Strategy trait for estimating gradients in a voxel grid.
pub trait GradientEstimator {
    /// Computes the gradient at a specific grid point.
    ///
    /// This is the safe method that handles boundary checks.
    fn gradient(&self, grid: &VoxelGrid, x: usize, y: usize, z: usize) -> Point3D;

    /// Computes the gradient at a specific index using direct memory access.
    ///
    /// # Safety
    /// This method is unsafe because it performs unchecked pointer arithmetic.
    /// The caller must ensure that `idx` is valid and that `idx ± stride_y` and `idx ± stride_z`
    /// are within the bounds of `data`.
    unsafe fn gradient_unchecked(
        &self,
        data: &[f32],
        idx: usize,
        stride_y: usize,
        stride_z: usize,
    ) -> Point3D;
}

/// Estimates gradients using the Central Difference method.
///
/// $ \nabla f \approx \left( \frac{f(x+1) - f(x-1)}{2}, \frac{f(y+1) - f(y-1)}{2}, \frac{f(z+1) - f(z-1)}{2} \right) $
pub struct CentralDifferenceEstimator;

impl GradientEstimator for CentralDifferenceEstimator {
    #[inline]
    fn gradient(&self, grid: &VoxelGrid, x: usize, y: usize, z: usize) -> Point3D {
        let dx = if x == 0 {
            grid.get(x + 1, y, z) - grid.get(x, y, z)
        } else if x == grid.width - 1 {
            grid.get(x, y, z) - grid.get(x - 1, y, z)
        } else {
            (grid.get(x + 1, y, z) - grid.get(x - 1, y, z)) / 2.0
        };

        let dy = if y == 0 {
            grid.get(x, y + 1, z) - grid.get(x, y, z)
        } else if y == grid.height - 1 {
            grid.get(x, y, z) - grid.get(x, y - 1, z)
        } else {
            (grid.get(x, y + 1, z) - grid.get(x, y - 1, z)) / 2.0
        };

        let dz = if z == 0 {
            grid.get(x, y, z + 1) - grid.get(x, y, z)
        } else if z == grid.depth - 1 {
            grid.get(x, y, z) - grid.get(x, y, z - 1)
        } else {
            (grid.get(x, y, z + 1) - grid.get(x, y, z - 1)) / 2.0
        };

        Point3D::new(dx, dy, dz)
    }

    #[inline(always)]
    unsafe fn gradient_unchecked(
        &self,
        data: &[f32],
        idx: usize,
        stride_y: usize,
        stride_z: usize,
    ) -> Point3D {
        // SAFETY: The caller guarantees idx is valid and has sufficient padding.
        unsafe {
            let dx = (*data.get_unchecked(idx + 1) - *data.get_unchecked(idx - 1)) * 0.5;
            let dy =
                (*data.get_unchecked(idx + stride_y) - *data.get_unchecked(idx - stride_y)) * 0.5;
            let dz =
                (*data.get_unchecked(idx + stride_z) - *data.get_unchecked(idx - stride_z)) * 0.5;
            Point3D::new(dx, dy, dz)
        }
    }
}
