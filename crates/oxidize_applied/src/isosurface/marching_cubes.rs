use super::error::IsosurfaceError;
use super::gradients::{CentralDifferenceEstimator, GradientEstimator};
use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE};
use super::types::{Mesh, Point3D, Triangle, VoxelGrid};

/// Interpolates between two points (p1, v1) and (p2, v2) to find the point where value == threshold.
#[inline]
fn interpolate(p1: Point3D, v1: f32, p2: Point3D, v2: f32, threshold: f32) -> Point3D {
    if (threshold - v1).abs() < Point3D::EPSILON {
        return p1;
    }
    if (threshold - v2).abs() < Point3D::EPSILON {
        return p2;
    }
    if (v1 - v2).abs() < Point3D::EPSILON {
        return p1;
    }

    let t = (threshold - v1) / (v2 - v1);
    p1 + (p2 - p1) * t
}

/// Linear interpolation of normals.
#[inline]
fn interpolate_normal(n1: Point3D, v1: f32, n2: Point3D, v2: f32, threshold: f32) -> Point3D {
    if (v1 - v2).abs() < Point3D::EPSILON {
        return n1;
    }
    let t = (threshold - v1) / (v2 - v1);
    let n = n1 + (n2 - n1) * t;
    n.normalize()
}

/// Data for a single cube in the grid.
struct CubeData {
    /// Values at the 8 corners.
    values: [f32; 8],
    /// Positions of the 8 corners.
    positions: [Point3D; 8],
    /// Normals at the 8 corners.
    normals: [Point3D; 8],
    /// The case index (0-255).
    index: usize,
}

/// Marching Cubes algorithm implementation.
///
/// Converts a scalar field (voxel grid) into a polygonal mesh.
pub struct MarchingCubes<'a, G: GradientEstimator> {
    grid: &'a VoxelGrid,
    estimator: G,
}

impl<'a, G: GradientEstimator> MarchingCubes<'a, G> {
    /// Creates a new Marching Cubes extractor.
    pub fn new(grid: &'a VoxelGrid, estimator: G) -> Self {
        Self { grid, estimator }
    }

    /// Extracts the isosurface for the given threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The scalar value at which to extract the surface. Values in the grid
    ///   less than this threshold are considered "inside" the volume, and values greater
    ///   than or equal to the threshold are considered "outside".
    ///
    /// # Returns
    ///
    /// Returns `Ok(Mesh)` containing the generated triangles on success.
    ///
    /// # Errors
    ///
    /// Returns an [`IsosurfaceError`] if the associated grid dimensions are invalid
    /// (e.g., less than 2x2x2) or if the data buffer length does not match the dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_applied::isosurface::{CentralDifferenceEstimator, VoxelGrid, Point3D};
    /// use oxidize_applied::isosurface::marching_cubes::MarchingCubes;
    ///
    /// // Define a minimal 2x2x2 grid.
    /// let grid = VoxelGrid {
    ///     width: 2, height: 2, depth: 2,
    ///     data: vec![
    ///         -1.0, -1.0,  // z=0, y=0
    ///         -1.0, -1.0,  // z=0, y=1
    ///          1.0,  1.0,  // z=1, y=0
    ///          1.0,  1.0   // z=1, y=1
    ///     ],
    ///     voxel_size: Point3D::new(1.0, 1.0, 1.0),
    ///     origin: Point3D::new(0.0, 0.0, 0.0),
    /// };
    ///
    /// // Instantiate the custom extractor.
    /// let extractor = MarchingCubes::new(&grid, CentralDifferenceEstimator);
    ///
    /// // Extract the surface at threshold 0.0.
    /// let mesh = extractor.extract(0.0).expect("Extraction failed");
    /// assert!(!mesh.triangles.is_empty(), "Expected a generated mesh");
    /// ```
    pub fn extract(&self, threshold: f32) -> Result<Mesh, IsosurfaceError> {
        self.validate_grid()?;

        // Estimate capacity
        let estimated_triangles = self.grid.width * self.grid.height * 5;
        let mut triangles = Vec::with_capacity(estimated_triangles);

        let stride_y = self.grid.width;
        let stride_z = self.grid.width * self.grid.height;

        // Iterate over each cube in the grid
        for z in 0..self.grid.depth - 1 {
            let z_interior = z > 0 && z < self.grid.depth - 2;

            for y in 0..self.grid.height - 1 {
                let y_interior = y > 0 && y < self.grid.height - 2;
                let row_is_interior = z_interior && y_interior;

                // Cache for the "Right Face" gradients of the previous iteration (x-1).
                let mut cached_gradients: Option<[Point3D; 4]> = None;

                for x in 0..self.grid.width - 1 {
                    let base_idx = z * stride_z + y * stride_y + x;

                    // 1. Get Values & Index
                    // SAFETY: `base_idx` is computed from `x,y,z` which are strictly less than `width-1`, `height-1`, `depth-1`.
                    // The offsets applied in `get_cube_values_fast` are at most `stride_y + stride_z + 1`.
                    // The maximum index accessed is `(depth-2)*stride_z + (height-2)*stride_y + (width-2) + stride_y + stride_z + 1`
                    // which equals `(depth-1)*stride_z + (height-1)*stride_y + (width-1)`.
                    // This is strictly less than `width * height * depth`, which is validated by `validate_grid` to be `<= data.len()`.
                    let (values, index) =
                        self.get_cube_values_fast(base_idx, stride_y, stride_z, threshold);

                    // 2. Early Exit if cube doesn't intersect surface
                    if CUBE_EDGE_FLAGS[index] == 0 {
                        cached_gradients = None;
                        continue;
                    }

                    // 3. Get Positions
                    let positions = self.get_cube_positions(x, y, z);

                    // 4. Get Normals (with caching)
                    let (normals, right_face) = self.compute_gradients_fast(
                        (x, y, z),
                        base_idx,
                        (stride_y, stride_z),
                        row_is_interior,
                        cached_gradients,
                    );
                    cached_gradients = Some(right_face);

                    let cube_data = CubeData {
                        values,
                        positions,
                        normals,
                        index,
                    };

                    // 5. Triangulate
                    self.triangulate_cube(&cube_data, threshold, &mut triangles);
                }
            }
        }

        Ok(Mesh { triangles })
    }

    fn validate_grid(&self) -> Result<(), IsosurfaceError> {
        if self.grid.width < 2 || self.grid.height < 2 || self.grid.depth < 2 {
            return Err(IsosurfaceError::InvalidGrid(
                "Grid dimensions must be at least 2x2x2".to_string(),
            ));
        }

        let expected_len = self
            .grid
            .width
            .checked_mul(self.grid.height)
            .and_then(|wh| wh.checked_mul(self.grid.depth))
            .ok_or_else(|| {
                IsosurfaceError::InvalidGrid("Grid dimensions cause integer overflow".to_string())
            })?;

        if self.grid.data.len() < expected_len {
            return Err(IsosurfaceError::DataMismatch {
                expected: expected_len,
                actual: self.grid.data.len(),
            });
        }
        Ok(())
    }

    /// Fetches the scalar values for the 8 corners of a cube and computes its case index.
    ///
    /// This method is highly optimized for the inner loop of the marching cubes algorithm.
    /// It uses unchecked indexing and bitwise operations to compute the active edges.
    ///
    #[inline(always)]
    fn get_cube_values_fast(
        &self,
        base_idx: usize,
        stride_y: usize,
        stride_z: usize,
        threshold: f32,
    ) -> ([f32; 8], usize) {
        let data = &self.grid.data;
        let mut values = [0.0; 8];
        let mut index = 0;

        // Helper macro to fetch and update index
        macro_rules! fetch {
            ($i:expr, $offset:expr, $bit:expr) => {
                let v = data.get(base_idx + $offset).copied().unwrap_or(0.0);
                values[$i] = v;
                if v < threshold {
                    index |= $bit;
                }
            };
        }

        fetch!(0, 0, 1);
        fetch!(1, 1, 2);
        fetch!(2, 1 + stride_y, 4);
        fetch!(3, stride_y, 8);
        fetch!(4, stride_z, 16);
        fetch!(5, 1 + stride_z, 32);
        fetch!(6, 1 + stride_y + stride_z, 64);
        fetch!(7, stride_y + stride_z, 128);

        (values, index)
    }

    #[inline]
    fn get_cube_positions(&self, x: usize, y: usize, z: usize) -> [Point3D; 8] {
        let x_pos = self.grid.origin.x + (x as f32) * self.grid.voxel_size.x;
        let y_pos = self.grid.origin.y + (y as f32) * self.grid.voxel_size.y;
        let z_pos = self.grid.origin.z + (z as f32) * self.grid.voxel_size.z;

        let next_x = x_pos + self.grid.voxel_size.x;
        let next_y = y_pos + self.grid.voxel_size.y;
        let next_z = z_pos + self.grid.voxel_size.z;

        [
            Point3D::new(x_pos, y_pos, z_pos),    // 0
            Point3D::new(next_x, y_pos, z_pos),   // 1
            Point3D::new(next_x, next_y, z_pos),  // 2
            Point3D::new(x_pos, next_y, z_pos),   // 3
            Point3D::new(x_pos, y_pos, next_z),   // 4
            Point3D::new(next_x, y_pos, next_z),  // 5
            Point3D::new(next_x, next_y, next_z), // 6
            Point3D::new(x_pos, next_y, next_z),  // 7
        ]
    }

    /// Computes the gradient (normal vectors) for the 8 corners of a cube, utilizing caching.
    ///
    /// This method implements a fast path for interior points where boundary conditions
    /// do not apply, avoiding branch predictions and bounds checks.
    ///
    #[inline(always)]
    fn compute_gradients_fast(
        &self,
        coords: (usize, usize, usize),
        base_idx: usize,
        strides: (usize, usize),
        row_is_interior: bool,
        cached_left_face: Option<[Point3D; 4]>,
    ) -> ([Point3D; 8], [Point3D; 4]) {
        let (x, y, z) = coords;
        let (stride_y, stride_z) = strides;
        let mut normals = [Point3D::new(0.0, 0.0, 0.0); 8];
        let data = &self.grid.data;

        // Check if X is interior (safe for fast path)
        let x_interior = x > 0 && x < self.grid.width - 2;
        let can_use_fast_path = row_is_interior && x_interior;

        // 1. Fill Left Face (0, 3, 4, 7)
        if let Some(grads) = cached_left_face {
            normals[0] = grads[0];
            normals[3] = grads[1];
            normals[4] = grads[2];
            normals[7] = grads[3];
        } else if can_use_fast_path {
            normals[0] = self
                .estimator
                .gradient_fast(data, base_idx, stride_y, stride_z);
            normals[3] =
                self.estimator
                    .gradient_fast(data, base_idx + stride_y, stride_y, stride_z);
            normals[4] =
                self.estimator
                    .gradient_fast(data, base_idx + stride_z, stride_y, stride_z);
            normals[7] = self.estimator.gradient_fast(
                data,
                base_idx + stride_y + stride_z,
                stride_y,
                stride_z,
            );
        } else {
            normals[0] = self.estimator.gradient(self.grid, x, y, z);
            normals[3] = self.estimator.gradient(self.grid, x, y + 1, z);
            normals[4] = self.estimator.gradient(self.grid, x, y, z + 1);
            normals[7] = self.estimator.gradient(self.grid, x, y + 1, z + 1);
        }

        // 2. Compute Right Face (1, 2, 5, 6)
        // These are always computed anew as they become the next Left Face
        let right_face = if can_use_fast_path {
            let next_x_idx = base_idx + 1;
            let n1 = self
                .estimator
                .gradient_fast(data, next_x_idx, stride_y, stride_z);
            let n2 = self
                .estimator
                .gradient_fast(data, next_x_idx + stride_y, stride_y, stride_z);
            let n5 = self
                .estimator
                .gradient_fast(data, next_x_idx + stride_z, stride_y, stride_z);
            let n6 = self.estimator.gradient_fast(
                data,
                next_x_idx + stride_y + stride_z,
                stride_y,
                stride_z,
            );
            [n1, n2, n5, n6]
        } else {
            let n1 = self.estimator.gradient(self.grid, x + 1, y, z);
            let n2 = self.estimator.gradient(self.grid, x + 1, y + 1, z);
            let n5 = self.estimator.gradient(self.grid, x + 1, y, z + 1);
            let n6 = self.estimator.gradient(self.grid, x + 1, y + 1, z + 1);
            [n1, n2, n5, n6]
        };

        normals[1] = right_face[0];
        normals[2] = right_face[1];
        normals[5] = right_face[2];
        normals[6] = right_face[3];

        (normals, right_face)
    }

    #[inline]
    fn triangulate_cube(&self, cube: &CubeData, threshold: f32, output: &mut Vec<Triangle>) {
        let edge_flags = CUBE_EDGE_FLAGS[cube.index];
        let mut edge_vertex = [Point3D::new(0.0, 0.0, 0.0); 12];
        let mut edge_norm = [Point3D::new(0.0, 0.0, 0.0); 12];

        // Interpolate vertices and normals on intersected edges
        for i in 0..12 {
            if (edge_flags & (1 << i)) != 0 {
                let v1_idx = EDGE_CONNECTION[i][0];
                let v2_idx = EDGE_CONNECTION[i][1];

                edge_vertex[i] = interpolate(
                    cube.positions[v1_idx],
                    cube.values[v1_idx],
                    cube.positions[v2_idx],
                    cube.values[v2_idx],
                    threshold,
                );

                edge_norm[i] = interpolate_normal(
                    cube.normals[v1_idx],
                    cube.values[v1_idx],
                    cube.normals[v2_idx],
                    cube.values[v2_idx],
                    threshold,
                );
            }
        }

        // Generate triangles using lookup table
        let mut i = 0;
        loop {
            let v1_lookup = TRIANGLE_CONNECTION_TABLE[cube.index][i];
            if v1_lookup == -1 {
                break;
            }
            let v2_lookup = TRIANGLE_CONNECTION_TABLE[cube.index][i + 1];
            let v3_lookup = TRIANGLE_CONNECTION_TABLE[cube.index][i + 2];

            let v1 = v1_lookup as usize;
            let v2 = v2_lookup as usize;
            let v3 = v3_lookup as usize;

            output.push(Triangle {
                v1: edge_vertex[v1],
                v2: edge_vertex[v2],
                v3: edge_vertex[v3],
                n1: edge_norm[v1],
                n2: edge_norm[v2],
                n3: edge_norm[v3],
            });

            i += 3;
        }
    }
}

/// Extracts the isosurface for the given threshold using the Marching Cubes algorithm.
///
/// Converts a scalar field (voxel grid) into a polygonal mesh using a central
/// difference estimator for normal calculation.
///
/// # Arguments
///
/// * `grid` - A reference to the [`VoxelGrid`] representing the volumetric data.
/// * `threshold` - The scalar value at which to extract the surface.
///
/// # Returns
///
/// Returns `Ok(Mesh)` containing the generated triangles on success, or
/// an `IsosurfaceError` if the grid is invalid or data length mismatches.
///
/// # Examples
///
/// ```
/// use oxidize_applied::isosurface::{extract_isosurface, VoxelGrid, Point3D};
///
/// // Create a tiny 2x2x2 grid representing a corner
/// let grid = VoxelGrid {
///     width: 2, height: 2, depth: 2,
///     data: vec![
///         -1.0, -1.0,  // z=0, y=0
///         -1.0, -1.0,  // z=0, y=1
///          1.0,  1.0,  // z=1, y=0
///          1.0,  1.0   // z=1, y=1
///     ],
///     voxel_size: Point3D::new(1.0, 1.0, 1.0),
///     origin: Point3D::new(0.0, 0.0, 0.0),
/// };
///
/// // Extract the surface where value == 0.0
/// let mesh = extract_isosurface(&grid, 0.0).expect("Failed to extract isosurface");
///
/// // The simple gradient will yield triangles spanning across the middle.
/// assert!(!mesh.triangles.is_empty(), "Mesh should contain triangles");
/// ```
pub fn extract_isosurface(grid: &VoxelGrid, threshold: f32) -> Result<Mesh, IsosurfaceError> {
    let extractor = MarchingCubes::new(grid, CentralDifferenceEstimator);
    extractor.extract(threshold)
}
