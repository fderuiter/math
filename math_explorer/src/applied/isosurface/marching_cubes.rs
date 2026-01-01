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

/// Calculates gradient using direct indexing without bounds checks.
/// Assumes that idx +/- 1, idx +/- stride_y, idx +/- stride_z are valid.
/// Only for use inside the safe interior zone of the grid.
#[inline(always)]
fn get_gradient_fast(data: &[f32], idx: usize, stride_y: usize, stride_z: usize) -> Point3D {
    // Safety: The caller must ensure idx is in the 'safe zone' (interior of the grid).
    // The safe zone logic in extract_isosurface guarantees:
    // idx +/- 1, idx +/- stride_y, idx +/- stride_z are all within data bounds.
    unsafe {
        let dx = (*data.get_unchecked(idx + 1) - *data.get_unchecked(idx - 1)) * 0.5;
        let dy = (*data.get_unchecked(idx + stride_y) - *data.get_unchecked(idx - stride_y)) * 0.5;
        let dz = (*data.get_unchecked(idx + stride_z) - *data.get_unchecked(idx - stride_z)) * 0.5;

        // Use a small epsilon for length squared to avoid sqrt of near-zero.
        // Original threshold was len > 1e-6, so len_sq > 1e-12.
        let len_sq = dx * dx + dy * dy + dz * dz;
        if len_sq > 1e-12 {
            let len = len_sq.sqrt();
            Point3D::new(dx / len, dy / len, dz / len)
        } else {
            Point3D::new(0.0, 0.0, 0.0)
        }
    }
}

/// Calculates the gradient at a grid point using central differences.
/// Safe version with full bounds checking.
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

    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len > 1e-6 {
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

        let safe_z = z >= 1 && z <= grid.depth - 3;

        for y in 0..grid.height - 1 {
            let zy_base = z_base + y * stride_y;
            let y_pos = grid.origin.y + (y as f32) * grid.voxel_size.y;
            let safe_y = y >= 1 && y <= grid.height - 3;

            // Profiler Optimization: Sliding Window
            // Pre-load the 'back face' (x=0) values.
            // Nodes: 0, 3, 4, 7
            let mut v0 = grid.data[zy_base];
            let mut v3 = grid.data[zy_base + stride_y];
            let mut v4 = grid.data[zy_base + stride_z];
            let mut v7 = grid.data[zy_base + stride_y + stride_z];

            // Compute initial back mask
            let mut back_mask = 0;
            if v0 < threshold { back_mask |= 1; }
            if v3 < threshold { back_mask |= 8; }
            if v4 < threshold { back_mask |= 16; }
            if v7 < threshold { back_mask |= 128; }

            for x in 0..grid.width - 1 {
                let base_idx = zy_base + x;

                // Load Front Face (nodes 1, 2, 5, 6)
                let v1 = grid.data[base_idx + 1];
                let v2 = grid.data[base_idx + 1 + stride_y];
                let v5 = grid.data[base_idx + 1 + stride_z];
                let v6 = grid.data[base_idx + 1 + stride_y + stride_z];

                let mut front_mask = 0;
                if v1 < threshold { front_mask |= 2; }
                if v2 < threshold { front_mask |= 4; }
                if v5 < threshold { front_mask |= 32; }
                if v6 < threshold { front_mask |= 64; }

                let cube_index = back_mask | front_mask;

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];
                if edge_flags == 0 {
                    // Shift window and continue
                    v0 = v1; v3 = v2; v4 = v5; v7 = v6;
                    back_mask = ((front_mask & 2) >> 1) |
                                ((front_mask & 4) << 1) |
                                ((front_mask & 32) >> 1) |
                                ((front_mask & 64) << 1);
                    continue;
                }

                // If active, we need full data
                let x_pos = grid.origin.x + (x as f32) * grid.voxel_size.x;
                let safe_x = x >= 1 && x <= grid.width - 3;

                // Construct full corner arrays
                let corner_values = [v0, v1, v2, v3, v4, v5, v6, v7];

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

                let mut corner_normals = [Point3D::new(0.0,0.0,0.0); 8];

                // Profiler Optimization: Lazy Gradient Computation
                let entirely_safe = safe_x && safe_y && safe_z;

                if entirely_safe {
                    corner_normals[0] = get_gradient_fast(&grid.data, base_idx, stride_y, stride_z);
                    corner_normals[1] = get_gradient_fast(&grid.data, base_idx + 1, stride_y, stride_z);
                    corner_normals[2] = get_gradient_fast(&grid.data, base_idx + 1 + stride_y, stride_y, stride_z);
                    corner_normals[3] = get_gradient_fast(&grid.data, base_idx + stride_y, stride_y, stride_z);
                    corner_normals[4] = get_gradient_fast(&grid.data, base_idx + stride_z, stride_y, stride_z);
                    corner_normals[5] = get_gradient_fast(&grid.data, base_idx + 1 + stride_z, stride_y, stride_z);
                    corner_normals[6] = get_gradient_fast(&grid.data, base_idx + 1 + stride_y + stride_z, stride_y, stride_z);
                    corner_normals[7] = get_gradient_fast(&grid.data, base_idx + stride_y + stride_z, stride_y, stride_z);
                } else {
                    for i in 0..8 {
                        let ox = x + VERTEX_OFFSET[i][0];
                        let oy = y + VERTEX_OFFSET[i][1];
                        let oz = z + VERTEX_OFFSET[i][2];
                        corner_normals[i] = get_gradient(grid, ox, oy, oz);
                    }
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

                // Shift for next iteration (must do this even if active)
                v0 = v1; v3 = v2; v4 = v5; v7 = v6;
                back_mask = ((front_mask & 2) >> 1) |
                            ((front_mask & 4) << 1) |
                            ((front_mask & 32) >> 1) |
                            ((front_mask & 64) << 1);
            }
        }
    }

    Ok(Mesh { triangles })
}
