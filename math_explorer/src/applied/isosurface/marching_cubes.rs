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

/// Optimized gradient calculation for INTERIOR points only.
#[inline(always)]
fn get_gradient_interior(
    data: &[f32],
    idx: usize,
    stride_y: usize,
    stride_z: usize
) -> Point3D {
    // Safety: Caller guarantees bounds.
    let dx = unsafe { *data.get_unchecked(idx + 1) - *data.get_unchecked(idx - 1) } * 0.5;
    let dy = unsafe { *data.get_unchecked(idx + stride_y) - *data.get_unchecked(idx - stride_y) } * 0.5;
    let dz = unsafe { *data.get_unchecked(idx + stride_z) - *data.get_unchecked(idx - stride_z) } * 0.5;

    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len > 1e-6 {
        Point3D::new(dx / len, dy / len, dz / len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
}

/// Safe gradient calculation with boundary checks.
#[inline]
fn get_gradient_safe(
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

    let estimated_triangles = grid.width * grid.height * 2;
    let mut triangles = Vec::with_capacity(estimated_triangles);

    let stride_y = grid.width;
    let stride_z = grid.width * grid.height;
    let width = grid.width;
    let height = grid.height;
    let depth = grid.depth;
    let data = &grid.data;

    for z in 0..depth - 1 {
        let z_base = z * stride_z;
        let z_pos = grid.origin.z + (z as f32) * grid.voxel_size.z;
        let z_safe = z > 0 && z < depth - 2;

        for y in 0..height - 1 {
            let zy_base = z_base + y * stride_y;
            let y_pos = grid.origin.y + (y as f32) * grid.voxel_size.y;
            let y_safe = y > 0 && y < height - 2;

            let row_is_safe = z_safe && y_safe;

            let mut cached_gradients: Option<[Point3D; 4]> = None;

            // Loop Splitting Logic
            if row_is_safe {
                // 1. Boundary x=0 (Slow Path)
                process_cube(0, zy_base, x_pos(0, grid), y_pos, z_pos, y, z,
                    stride_y, stride_z, width, height, depth,
                    data, grid, threshold, &mut triangles, &mut cached_gradients, false);

                // 2. Interior (Fast Path)
                // x goes from 1 to width-2 (inclusive start, exclusive end)
                for x in 1..width - 2 {
                    process_cube(x, zy_base + x, x_pos(x, grid), y_pos, z_pos, y, z,
                        stride_y, stride_z, width, height, depth,
                        data, grid, threshold, &mut triangles, &mut cached_gradients, true);
                }

                // 3. Boundary x=width-2 (Slow Path)
                // If width <= 2, the interior loop 1..width-2 is empty, and we might double process x=0.
                // Since x loop stops at width-2, if width > 2, the last x is width-2.
                if width > 2 {
                     process_cube(width - 2, zy_base + width - 2, x_pos(width - 2, grid), y_pos, z_pos, y, z,
                        stride_y, stride_z, width, height, depth,
                        data, grid, threshold, &mut triangles, &mut cached_gradients, false);
                }

            } else {
                // Whole row is unsafe (either y or z is boundary)
                for x in 0..width - 1 {
                    process_cube(x, zy_base + x, x_pos(x, grid), y_pos, z_pos, y, z,
                        stride_y, stride_z, width, height, depth,
                        data, grid, threshold, &mut triangles, &mut cached_gradients, false);
                }
            }
        }
    }

    Ok(Mesh { triangles })
}

#[inline(always)]
fn x_pos(x: usize, grid: &VoxelGrid) -> f32 {
    grid.origin.x + (x as f32) * grid.voxel_size.x
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn process_cube(
    x: usize, base_idx: usize, x_pos: f32, y_pos: f32, z_pos: f32,
    y_idx: usize, z_idx: usize, // Passed for slow path
    stride_y: usize, stride_z: usize, width: usize, height: usize, depth: usize,
    data: &[f32], grid: &VoxelGrid, threshold: f32,
    triangles: &mut Vec<Triangle>,
    cached_gradients: &mut Option<[Point3D; 4]>,
    fast_path: bool
) {
     // 1. Determine the index of the case (0-255)
    let mut cube_index = 0;

    // Access values - safe to do normally as it's just array indexing
    let v0 = data[base_idx];
    let v1 = data[base_idx + 1];
    let v2 = data[base_idx + 1 + stride_y];
    let v3 = data[base_idx + stride_y];
    let v4 = data[base_idx + stride_z];
    let v5 = data[base_idx + 1 + stride_z];
    let v6 = data[base_idx + 1 + stride_y + stride_z];
    let v7 = data[base_idx + stride_y + stride_z];

    if v0 < threshold { cube_index |= 1; }
    if v1 < threshold { cube_index |= 2; }
    if v2 < threshold { cube_index |= 4; }
    if v3 < threshold { cube_index |= 8; }
    if v4 < threshold { cube_index |= 16; }
    if v5 < threshold { cube_index |= 32; }
    if v6 < threshold { cube_index |= 64; }
    if v7 < threshold { cube_index |= 128; }

    let edge_flags = CUBE_EDGE_FLAGS[cube_index];
    if edge_flags == 0 {
        *cached_gradients = None;
        return;
    }

    // Calculate positions
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
        Point3D::new(x_pos, next_y_pos, next_z_pos)
    ];

    let corner_values = [v0, v1, v2, v3, v4, v5, v6, v7];

    let mut corner_normals = [Point3D::new(0.0,0.0,0.0); 8];

    // 1. Fill Left Face (0, 3, 4, 7)
    if let Some(grads) = cached_gradients {
        corner_normals[0] = grads[0];
        corner_normals[3] = grads[1];
        corner_normals[4] = grads[2];
        corner_normals[7] = grads[3];
    } else {
        if fast_path {
            corner_normals[0] = get_gradient_interior(data, base_idx, stride_y, stride_z);
            corner_normals[3] = get_gradient_interior(data, base_idx + stride_y, stride_y, stride_z);
            corner_normals[4] = get_gradient_interior(data, base_idx + stride_z, stride_y, stride_z);
            corner_normals[7] = get_gradient_interior(data, base_idx + stride_y + stride_z, stride_y, stride_z);
        } else {
             corner_normals[0] = get_gradient_safe(data, base_idx, x, y_idx, z_idx, width, height, depth, stride_y, stride_z);
             corner_normals[3] = get_gradient_safe(data, base_idx + stride_y, x, y_idx+1, z_idx, width, height, depth, stride_y, stride_z);
             corner_normals[4] = get_gradient_safe(data, base_idx + stride_z, x, y_idx, z_idx+1, width, height, depth, stride_y, stride_z);
             corner_normals[7] = get_gradient_safe(data, base_idx + stride_y + stride_z, x, y_idx+1, z_idx+1, width, height, depth, stride_y, stride_z);
        }
    }

    // 2. Compute Right Face (1, 2, 5, 6)
    if fast_path {
            corner_normals[1] = get_gradient_interior(data, base_idx + 1, stride_y, stride_z);
            corner_normals[2] = get_gradient_interior(data, base_idx + 1 + stride_y, stride_y, stride_z);
            corner_normals[5] = get_gradient_interior(data, base_idx + 1 + stride_z, stride_y, stride_z);
            corner_normals[6] = get_gradient_interior(data, base_idx + 1 + stride_y + stride_z, stride_y, stride_z);
    } else {
            corner_normals[1] = get_gradient_safe(data, base_idx + 1, x+1, y_idx, z_idx, width, height, depth, stride_y, stride_z);
            corner_normals[2] = get_gradient_safe(data, base_idx + 1 + stride_y, x+1, y_idx+1, z_idx, width, height, depth, stride_y, stride_z);
            corner_normals[5] = get_gradient_safe(data, base_idx + 1 + stride_z, x+1, y_idx, z_idx+1, width, height, depth, stride_y, stride_z);
            corner_normals[6] = get_gradient_safe(data, base_idx + 1 + stride_y + stride_z, x+1, y_idx+1, z_idx+1, width, height, depth, stride_y, stride_z);
    }

    *cached_gradients = Some([
        corner_normals[1],
        corner_normals[2],
        corner_normals[5],
        corner_normals[6],
    ]);

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
