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

/// Calculates the gradient at a grid point using central differences.
/// optimized to avoid redundant index calculations and bounds checks.
#[inline]
fn get_gradient_fast(
    grid: &VoxelGrid,
    idx: usize,
    x: usize,
    y: usize,
    z: usize,
    stride_y: usize,
    stride_z: usize
) -> Point3D {
    let dx = if x == 0 {
        grid.data[idx + 1] - grid.data[idx]
    } else if x == grid.width - 1 {
        grid.data[idx] - grid.data[idx - 1]
    } else {
        (grid.data[idx + 1] - grid.data[idx - 1]) * 0.5
    };

    let dy = if y == 0 {
        grid.data[idx + stride_y] - grid.data[idx]
    } else if y == grid.height - 1 {
        grid.data[idx] - grid.data[idx - stride_y]
    } else {
        (grid.data[idx + stride_y] - grid.data[idx - stride_y]) * 0.5
    };

    let dz = if z == 0 {
        grid.data[idx + stride_z] - grid.data[idx]
    } else if z == grid.depth - 1 {
        grid.data[idx] - grid.data[idx - stride_z]
    } else {
        (grid.data[idx + stride_z] - grid.data[idx - stride_z]) * 0.5
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

            // Initialize left slice values (for x=0)
            // Left slice vertices: 0, 3, 4, 7 (indices relative to cube)
            // In global memory: base, base+sy, base+sz, base+sy+sz

            // Note: We use raw indices for speed.
            let idx_0 = zy_base; // Vertex 0
            let idx_3 = zy_base + stride_y; // Vertex 3
            let idx_4 = zy_base + stride_z; // Vertex 4
            let idx_7 = zy_base + stride_y + stride_z; // Vertex 7

            let mut v0 = grid.data[idx_0];
            let mut v3 = grid.data[idx_3];
            let mut v4 = grid.data[idx_4];
            let mut v7 = grid.data[idx_7];

            // Calculate initial left mask
            let mut left_mask = 0u8;
            if v0 < threshold { left_mask |= 1; }
            if v3 < threshold { left_mask |= 8; }
            if v4 < threshold { left_mask |= 16; }
            if v7 < threshold { left_mask |= 128; }

            for x in 0..grid.width - 1 {
                let base_idx = zy_base + x;
                let x_pos = grid.origin.x + (x as f32) * grid.voxel_size.x;

                // Load right slice (becomes vertices 1, 2, 5, 6)
                // x+1 corresponds to the right side of the current cube
                let right_base_idx = base_idx + 1;

                let v1 = grid.data[right_base_idx];
                let v2 = grid.data[right_base_idx + stride_y];
                let v5 = grid.data[right_base_idx + stride_z];
                let v6 = grid.data[right_base_idx + stride_y + stride_z];

                let mut right_mask = 0u8;
                if v1 < threshold { right_mask |= 2; } // Bit 1
                if v2 < threshold { right_mask |= 4; } // Bit 2
                if v5 < threshold { right_mask |= 32; } // Bit 5
                if v6 < threshold { right_mask |= 64; } // Bit 6

                // Combine masks to get full cube index
                let cube_index = (left_mask | right_mask) as usize;

                // Check if surface intersects this cube
                if CUBE_EDGE_FLAGS[cube_index] != 0 {
                    let mut corner_values = [0.0; 8];
                    corner_values[0] = v0;
                    corner_values[1] = v1;
                    corner_values[2] = v2;
                    corner_values[3] = v3;
                    corner_values[4] = v4;
                    corner_values[5] = v5;
                    corner_values[6] = v6;
                    corner_values[7] = v7;

                    // Positions
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
                    for i in 0..8 {
                        let ox = x + VERTEX_OFFSET[i][0];
                        let oy = y + VERTEX_OFFSET[i][1];
                        let oz = z + VERTEX_OFFSET[i][2];

                        let offset = VERTEX_OFFSET[i][0] + VERTEX_OFFSET[i][1] * stride_y + VERTEX_OFFSET[i][2] * stride_z;
                        let o_idx = base_idx + offset;

                        corner_normals[i] = get_gradient_fast(grid, o_idx, ox, oy, oz, stride_y, stride_z);
                    }

                    // 3. Compute intersection points
                    let edge_flags = CUBE_EDGE_FLAGS[cube_index];
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

                // Advance sliding window
                // Right slice becomes next Left slice
                // Mapping:
                // v1 (1,0,0) -> v0 (0,0,0) [Bit 1 -> Bit 0]
                // v2 (1,1,0) -> v3 (0,1,0) [Bit 2 -> Bit 3]
                // v5 (1,0,1) -> v4 (0,0,1) [Bit 5 -> Bit 4]
                // v6 (1,1,1) -> v7 (0,1,1) [Bit 6 -> Bit 7]

                v0 = v1;
                v3 = v2;
                v4 = v5;
                v7 = v6;

                // Shift mask bits
                // Bit 1 (0x02) -> Bit 0 (0x01): >> 1
                // Bit 2 (0x04) -> Bit 3 (0x08): << 1
                // Bit 5 (0x20) -> Bit 4 (0x10): >> 1
                // Bit 6 (0x40) -> Bit 7 (0x80): << 1
                left_mask = ((right_mask & 2) >> 1) |
                            ((right_mask & 4) << 1) |
                            ((right_mask & 32) >> 1) |
                            ((right_mask & 64) << 1);
            }
        }
    }

    Ok(Mesh { triangles })
}
