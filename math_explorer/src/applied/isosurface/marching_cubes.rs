use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE, VERTEX_OFFSET};
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

/// Calculates the gradient at a grid point using direct slice access.
/// This avoids repeated index calculations and bounds checking overhead of `grid.get()`.
/// `idx` must be the correct index for `(x, y, z)` in `data`.
#[inline]
fn get_gradient_fast(
    data: &[f32],
    idx: usize,
    x: usize, y: usize, z: usize,
    width: usize, height: usize, depth: usize,
    stride_y: usize, stride_z: usize
) -> Point3D {
    let dx = if x == 0 {
        data[idx + 1] - data[idx]
    } else if x == width - 1 {
        data[idx] - data[idx - 1]
    } else {
        (data[idx + 1] - data[idx - 1]) * 0.5
    };

    let dy = if y == 0 {
        data[idx + stride_y] - data[idx]
    } else if y == height - 1 {
        data[idx] - data[idx - stride_y]
    } else {
        (data[idx + stride_y] - data[idx - stride_y]) * 0.5
    };

    let dz = if z == 0 {
        data[idx + stride_z] - data[idx]
    } else if z == depth - 1 {
        data[idx] - data[idx - stride_z]
    } else {
        (data[idx + stride_z] - data[idx - stride_z]) * 0.5
    };

    let len_sq = dx * dx + dy * dy + dz * dz;
    if len_sq > 1e-12 {
        let len = len_sq.sqrt();
        Point3D::new(dx / len, dy / len, dz / len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
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

    let len = (nx*nx + ny*ny + nz*nz).sqrt();
    if len > 1e-6 {
        Point3D::new(nx/len, ny/len, nz/len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
}


pub fn extract_isosurface(grid: &VoxelGrid, threshold: f32) -> Result<Mesh, String> {
    if grid.width < 2 || grid.height < 2 || grid.depth < 2 {
        return Err("Grid dimensions must be at least 2x2x2".to_string());
    }

    // Estimate capacity to avoid reallocations
    // A heuristic: surface area roughly scales with N^2.
    // Let's reserve enough for a sphere of radius N/3.
    let estimated_triangles = grid.width * grid.height * 2;
    let mut triangles = Vec::with_capacity(estimated_triangles);

    let stride_y = grid.width;
    let stride_z = grid.width * grid.height;

    // Iterate over each cube in the grid
    for z in 0..grid.depth - 1 {
        let z_base = z * stride_z;
        let z_pos = grid.origin.z + (z as f32) * grid.voxel_size.z;

        for y in 0..grid.height - 1 {
            let zy_base = z_base + y * stride_y;
            let y_pos = grid.origin.y + (y as f32) * grid.voxel_size.y;

            // Optimization: Sliding Window
            // We maintain the 'left face' values (x) and load 'right face' values (x+1).
            // This reduces memory fetches from 8 to 4 per voxel and allows skipping
            // coordinate calculations for empty space.

            // Indices for the 4 rows involved in this scanline
            let r0 = zy_base;
            let r3 = zy_base + stride_y;
            let r4 = zy_base + stride_z;
            let r7 = zy_base + stride_z + stride_y;

            // Pre-load left face values for x=0
            let mut val0 = grid.data[r0];
            let mut val3 = grid.data[r3];
            let mut val4 = grid.data[r4];
            let mut val7 = grid.data[r7];

            for x in 0..grid.width - 1 {
                // Fetch right face values (corresponding to x+1)
                // Accessing linearly along the row allows for efficient prefetching
                let val1 = grid.data[r0 + x + 1];
                let val2 = grid.data[r3 + x + 1];
                let val5 = grid.data[r4 + x + 1];
                let val6 = grid.data[r7 + x + 1];

                // 1. Determine index of the case (0-255)
                let mut cube_index = 0;
                if val0 < threshold { cube_index |= 1; }
                if val1 < threshold { cube_index |= 2; }
                if val2 < threshold { cube_index |= 4; }
                if val3 < threshold { cube_index |= 8; }
                if val4 < threshold { cube_index |= 16; }
                if val5 < threshold { cube_index |= 32; }
                if val6 < threshold { cube_index |= 64; }
                if val7 < threshold { cube_index |= 128; }

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];

                if edge_flags != 0 {
                    // Only perform heavy allocations and calculations if surface intersects

                    let base_idx = zy_base + x;
                    let x_pos = grid.origin.x + (x as f32) * grid.voxel_size.x;
                    let corner_values = [val0, val1, val2, val3, val4, val5, val6, val7];

                    let next_x_pos = x_pos + grid.voxel_size.x;
                    let next_y_pos = y_pos + grid.voxel_size.y;
                    let next_z_pos = z_pos + grid.voxel_size.z;

                    let corner_pos = [
                        Point3D::new(x_pos, y_pos, z_pos),
                        Point3D::new(next_x_pos, y_pos, z_pos),
                        Point3D::new(next_x_pos, next_y_pos, z_pos),
                        Point3D::new(x_pos, next_y_pos, z_pos),
                        Point3D::new(x_pos, y_pos, next_z_pos),
                        Point3D::new(next_x_pos, y_pos, next_z_pos),
                        Point3D::new(next_x_pos, next_y_pos, next_z_pos),
                        Point3D::new(x_pos, next_y_pos, next_z_pos),
                    ];

                    // Precompute corner indices for gradient lookup
                    let corner_indices = [
                        base_idx,
                        base_idx + 1,
                        base_idx + 1 + stride_y,
                        base_idx + stride_y,
                        base_idx + stride_z,
                        base_idx + 1 + stride_z,
                        base_idx + 1 + stride_y + stride_z,
                        base_idx + stride_y + stride_z,
                    ];

                    let mut corner_normals = [Point3D::new(0.0,0.0,0.0); 8];
                    for i in 0..8 {
                        let ox = x + VERTEX_OFFSET[i][0];
                        let oy = y + VERTEX_OFFSET[i][1];
                        let oz = z + VERTEX_OFFSET[i][2];

                        corner_normals[i] = get_gradient_fast(
                            &grid.data,
                            corner_indices[i],
                            ox, oy, oz,
                            grid.width, grid.height, grid.depth,
                            stride_y, stride_z
                        );
                    }

                    // 3. Compute intersection points on required edges
                    let mut edge_vertex = [Point3D::new(0.0,0.0,0.0); 12];
                    let mut edge_norm = [Point3D::new(0.0,0.0,0.0); 12];

                    for i in 0..12 {
                        if (edge_flags & (1 << i)) != 0 {
                            let v1_idx = EDGE_CONNECTION[i][0];
                            let v2_idx = EDGE_CONNECTION[i][1];

                            edge_vertex[i] = interpolate(
                                corner_pos[v1_idx], corner_values[v1_idx],
                                corner_pos[v2_idx], corner_values[v2_idx],
                                threshold
                            );

                             edge_norm[i] = interpolate_normal(
                                corner_normals[v1_idx], corner_values[v1_idx],
                                corner_normals[v2_idx], corner_values[v2_idx],
                                threshold
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

                // Shift right face to left face for next iteration
                val0 = val1;
                val3 = val2;
                val4 = val5;
                val7 = val6;
            }
        }
    }

    Ok(Mesh { triangles })
}
