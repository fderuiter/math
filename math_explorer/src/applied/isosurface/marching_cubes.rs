use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE};
use super::types::{Mesh, Point3D, Triangle, VoxelGrid};

/// Interpolates between two points (p1, v1) and (p2, v2) to find the point where value == threshold.
#[inline]
fn interpolate(p1: Point3D, v1: f32, p2: Point3D, v2: f32, threshold: f32) -> Point3D {
    if (threshold - v1).abs() < 1e-5 {
        return p1;
    }
    if (threshold - v2).abs() < 1e-5 {
        return p2;
    }
    if (v1 - v2).abs() < 1e-5 {
        return p1;
    }

    let t = (threshold - v1) / (v2 - v1);
    Point3D::new(
        p1.x + t * (p2.x - p1.x),
        p1.y + t * (p2.y - p1.y),
        p1.z + t * (p2.z - p1.z),
    )
}

/// Calculates the gradient at a grid point using central differences.
#[inline]
fn get_gradient(grid: &VoxelGrid, x: usize, y: usize, z: usize) -> Point3D {
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

    // Optimization: Return un-normalized gradient. Normalization happens after interpolation.
    Point3D::new(dx, dy, dz)
}

/// Optimized gradient calculation for interior points.
///
/// # Safety
/// Caller must ensure that `idx` is sufficiently far from the start/end of the buffer
/// such that `idx ± stride_z` are valid indices.
/// Specifically: `z > 0 && z < depth-1`, `y > 0 && y < height-1`, `x > 0 && x < width-1`.
#[inline(always)]
fn get_gradient_interior(data: &[f32], idx: usize, stride_y: usize, stride_z: usize) -> Point3D {
    // Profiler Note: Using unsafe get_unchecked significantly reduces overhead by skipping bounds checks.
    // We trust the caller (extract_isosurface) to only call this for strictly interior voxels.
    let dx = unsafe { (*data.get_unchecked(idx + 1) - *data.get_unchecked(idx - 1)) * 0.5 };
    let dy = unsafe {
        (*data.get_unchecked(idx + stride_y) - *data.get_unchecked(idx - stride_y)) * 0.5
    };
    let dz = unsafe {
        (*data.get_unchecked(idx + stride_z) - *data.get_unchecked(idx - stride_z)) * 0.5
    };

    // Optimization: Return un-normalized gradient to save sqrt ops.
    Point3D::new(dx, dy, dz)
}

/// Linear interpolation of normals.
#[inline]
fn interpolate_normal(n1: Point3D, v1: f32, n2: Point3D, v2: f32, threshold: f32) -> Point3D {
    if (v1 - v2).abs() < 1e-5 {
        return n1;
    }
    let t = (threshold - v1) / (v2 - v1);
    let nx = n1.x + t * (n2.x - n1.x);
    let ny = n1.y + t * (n2.y - n1.y);
    let nz = n1.z + t * (n2.z - n1.z);

    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 1e-6 {
        Point3D::new(nx / len, ny / len, nz / len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
}

pub fn extract_isosurface(grid: &VoxelGrid, threshold: f32) -> Result<Mesh, String> {
    if grid.width < 2 || grid.height < 2 || grid.depth < 2 {
        return Err("Grid dimensions must be at least 2x2x2".to_string());
    }

    // Safety Check: Ensure data buffer is sufficient to prevent OOB access in unsafe blocks
    let expected_len = grid
        .width
        .checked_mul(grid.height)
        .and_then(|wh| wh.checked_mul(grid.depth))
        .ok_or_else(|| "Grid dimensions cause integer overflow".to_string())?;

    if grid.data.len() < expected_len {
        return Err(format!(
            "Data buffer size mismatch. Expected at least {}, got {}",
            expected_len,
            grid.data.len()
        ));
    }

    // Estimate capacity to avoid reallocations
    // A heuristic: surface area roughly scales with N^2.
    // Let's reserve enough for a sphere of radius N/3.
    let estimated_triangles = grid.width * grid.height * 2;
    let mut triangles = Vec::with_capacity(estimated_triangles);

    let stride_y = grid.width;
    let stride_z = grid.width * grid.height;
    let data = &grid.data;

    // Iterate over each cube in the grid
    for z in 0..grid.depth - 1 {
        let z_base = z * stride_z;
        let z_pos = grid.origin.z + (z as f32) * grid.voxel_size.z;

        // Check if Z slice is interior (for gradient calculation of this and next slice)
        // We need gradients at z and z+1.
        // For z (Current Slice): Needs z-1 available (z > 0).
        // For z+1 (Next Slice): Needs z+2 available (z+1 < depth-1 => z < depth-2).
        let z_interior = z > 0 && z < grid.depth - 2;

        for y in 0..grid.height - 1 {
            let zy_base = z_base + y * stride_y;
            let y_pos = grid.origin.y + (y as f32) * grid.voxel_size.y;

            // Check if Y row is interior
            // Similarly, we need gradients at y and y+1.
            // y > 0 && y < height - 2.
            let y_interior = y > 0 && y < grid.height - 2;

            // Combined interior check for Z and Y
            let row_is_interior = z_interior && y_interior;

            // Cache for the "Right Face" gradients of the previous iteration (x-1).
            // Corresponds to vertices 1, 2, 5, 6 of (x-1), which become 0, 3, 4, 7 of (x).
            let mut cached_gradients: Option<[Point3D; 4]> = None;

            for x in 0..grid.width - 1 {
                let base_idx = zy_base + x;
                let x_pos = grid.origin.x + (x as f32) * grid.voxel_size.x;

                // 1. Determine the index of the case (0-255)
                let mut cube_index = 0;
                let mut corner_values = [0.0; 8];
                let mut corner_pos = [Point3D::new(0.0, 0.0, 0.0); 8];
                let mut corner_normals = [Point3D::new(0.0, 0.0, 0.0); 8];

                // Direct access for corner values to avoid redundant index calculation
                // Vertices are ordered:
                // 0: (0,0,0), 1: (1,0,0), 2: (1,1,0), 3: (0,1,0)
                // 4: (0,0,1), 5: (1,0,1), 6: (1,1,1), 7: (0,1,1)

                // Safety: We are iterating up to width-1, height-1, depth-1.
                // Max index accesses base_idx + 1 + stride_y + stride_z.
                // base_idx = z*Sz + y*Sy + x.
                // Max = (D-2)*Sz + (H-2)*Sy + (W-2) + 1 + Sy + Sz
                //     = (D-1)*Sz + (H-1)*Sy + W-1
                // Which is exactly the last element. So indices are valid.
                let v0 = unsafe { *data.get_unchecked(base_idx) };
                let v1 = unsafe { *data.get_unchecked(base_idx + 1) };
                let v2 = unsafe { *data.get_unchecked(base_idx + 1 + stride_y) };
                let v3 = unsafe { *data.get_unchecked(base_idx + stride_y) };
                let v4 = unsafe { *data.get_unchecked(base_idx + stride_z) };
                let v5 = unsafe { *data.get_unchecked(base_idx + 1 + stride_z) };
                let v6 = unsafe { *data.get_unchecked(base_idx + 1 + stride_y + stride_z) };
                let v7 = unsafe { *data.get_unchecked(base_idx + stride_y + stride_z) };

                corner_values[0] = v0;
                if v0 < threshold {
                    cube_index |= 1;
                }
                corner_values[1] = v1;
                if v1 < threshold {
                    cube_index |= 2;
                }
                corner_values[2] = v2;
                if v2 < threshold {
                    cube_index |= 4;
                }
                corner_values[3] = v3;
                if v3 < threshold {
                    cube_index |= 8;
                }
                corner_values[4] = v4;
                if v4 < threshold {
                    cube_index |= 16;
                }
                corner_values[5] = v5;
                if v5 < threshold {
                    cube_index |= 32;
                }
                corner_values[6] = v6;
                if v6 < threshold {
                    cube_index |= 64;
                }
                corner_values[7] = v7;
                if v7 < threshold {
                    cube_index |= 128;
                }

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];
                if edge_flags == 0 {
                    cached_gradients = None; // Invalidate cache since we aren't computing gradients for this cube
                    continue;
                }

                // Only compute positions if needed
                let next_x_pos = x_pos + grid.voxel_size.x;
                let next_y_pos = y_pos + grid.voxel_size.y;
                let next_z_pos = z_pos + grid.voxel_size.z;

                corner_pos[0] = Point3D::new(x_pos, y_pos, z_pos);
                corner_pos[1] = Point3D::new(next_x_pos, y_pos, z_pos);
                corner_pos[2] = Point3D::new(next_x_pos, next_y_pos, z_pos);
                corner_pos[3] = Point3D::new(x_pos, next_y_pos, z_pos);
                corner_pos[4] = Point3D::new(x_pos, y_pos, next_z_pos);
                corner_pos[5] = Point3D::new(next_x_pos, y_pos, next_z_pos);
                corner_pos[6] = Point3D::new(next_x_pos, next_y_pos, next_z_pos);
                corner_pos[7] = Point3D::new(x_pos, next_y_pos, next_z_pos);

                // Profiler Optimization: Lazy Gradient Computation & Sliding Window

                // Check if X is interior.
                // We need gradients at x and x+1.
                // x > 0 && x < width - 2.
                let x_interior = x > 0 && x < grid.width - 2;

                let can_use_fast_path = row_is_interior && x_interior;

                // 1. Fill Left Face (0, 3, 4, 7) from cache or compute
                if let Some(grads) = cached_gradients {
                    corner_normals[0] = grads[0];
                    corner_normals[3] = grads[1];
                    corner_normals[4] = grads[2];
                    corner_normals[7] = grads[3];
                } else if can_use_fast_path {
                    corner_normals[0] = get_gradient_interior(data, base_idx, stride_y, stride_z);
                    corner_normals[3] =
                        get_gradient_interior(data, base_idx + stride_y, stride_y, stride_z);
                    corner_normals[4] =
                        get_gradient_interior(data, base_idx + stride_z, stride_y, stride_z);
                    corner_normals[7] = get_gradient_interior(
                        data,
                        base_idx + stride_y + stride_z,
                        stride_y,
                        stride_z,
                    );
                } else {
                    corner_normals[0] = get_gradient(grid, x, y, z);
                    corner_normals[3] = get_gradient(grid, x, y + 1, z);
                    corner_normals[4] = get_gradient(grid, x, y, z + 1);
                    corner_normals[7] = get_gradient(grid, x, y + 1, z + 1);
                }

                // 2. Compute Right Face (1, 2, 5, 6) - these are always new
                // Vertices:
                // 1: (x+1, y, z)
                // 2: (x+1, y+1, z)
                // 5: (x+1, y, z+1)
                // 6: (x+1, y+1, z+1)

                if can_use_fast_path {
                    let next_x_idx = base_idx + 1;
                    corner_normals[1] = get_gradient_interior(data, next_x_idx, stride_y, stride_z);
                    corner_normals[2] =
                        get_gradient_interior(data, next_x_idx + stride_y, stride_y, stride_z);
                    corner_normals[5] =
                        get_gradient_interior(data, next_x_idx + stride_z, stride_y, stride_z);
                    corner_normals[6] = get_gradient_interior(
                        data,
                        next_x_idx + stride_y + stride_z,
                        stride_y,
                        stride_z,
                    );
                } else {
                    corner_normals[1] = get_gradient(grid, x + 1, y, z);
                    corner_normals[2] = get_gradient(grid, x + 1, y + 1, z);
                    corner_normals[5] = get_gradient(grid, x + 1, y, z + 1);
                    corner_normals[6] = get_gradient(grid, x + 1, y + 1, z + 1);
                }

                // 3. Update cache for next iteration (which will use these as Left Face)
                cached_gradients = Some([
                    corner_normals[1],
                    corner_normals[2],
                    corner_normals[5],
                    corner_normals[6],
                ]);

                // 3. Compute intersection points on required edges
                let mut edge_vertex = [Point3D::new(0.0, 0.0, 0.0); 12];
                let mut edge_norm = [Point3D::new(0.0, 0.0, 0.0); 12];

                for i in 0..12 {
                    if (edge_flags & (1 << i)) != 0 {
                        let v1_idx = EDGE_CONNECTION[i][0];
                        let v2_idx = EDGE_CONNECTION[i][1];

                        edge_vertex[i] = interpolate(
                            corner_pos[v1_idx],
                            corner_values[v1_idx],
                            corner_pos[v2_idx],
                            corner_values[v2_idx],
                            threshold,
                        );

                        edge_norm[i] = interpolate_normal(
                            corner_normals[v1_idx],
                            corner_values[v1_idx],
                            corner_normals[v2_idx],
                            corner_values[v2_idx],
                            threshold,
                        );
                    }
                }

                // 4. Create triangles
                let mut i = 0;
                while TRIANGLE_CONNECTION_TABLE[cube_index][i] != -1 {
                    let v1 = TRIANGLE_CONNECTION_TABLE[cube_index][i] as usize;
                    let v2 = TRIANGLE_CONNECTION_TABLE[cube_index][i + 1] as usize;
                    let v3 = TRIANGLE_CONNECTION_TABLE[cube_index][i + 2] as usize;

                    triangles.push(Triangle {
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
    }

    Ok(Mesh { triangles })
}
