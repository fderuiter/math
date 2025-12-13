use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE, VERTEX_OFFSET};
use super::types::{Mesh, Point3D, Triangle, VoxelGrid};

/// Interpolates between two points (p1, v1) and (p2, v2) to find the point where value == threshold.
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

    // The gradient points from low values to high values.
    // Inside: V < Threshold, Outside: V >= Threshold.
    // So gradient points OUTWARD.
    // The surface normal is usually defined as pointing outward.
    // Thus, Normal = Normalized Gradient.
    // (Previous implementation inverted this, fixed now).

    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len > 1e-6 {
        Point3D::new(dx / len, dy / len, dz / len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
}

/// Linear interpolation of normals.
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

    let mut triangles = Vec::new();

    // Iterate over each cube in the grid
    for z in 0..grid.depth - 1 {
        for y in 0..grid.height - 1 {
            for x in 0..grid.width - 1 {

                // 1. Determine the index of the case (0-255)
                let mut cube_index = 0;
                let mut corner_values = [0.0; 8];
                let mut corner_pos = [Point3D::new(0.0,0.0,0.0); 8];
                let mut corner_normals = [Point3D::new(0.0,0.0,0.0); 8];

                for i in 0..8 {
                    let ox = x + VERTEX_OFFSET[i][0];
                    let oy = y + VERTEX_OFFSET[i][1];
                    let oz = z + VERTEX_OFFSET[i][2];

                    let val = grid.get(ox, oy, oz);
                    corner_values[i] = val;

                    if val < threshold {
                        cube_index |= 1 << i;
                    }

                    corner_pos[i] = Point3D::new(
                        grid.origin.x + (ox as f32) * grid.voxel_size.x,
                        grid.origin.y + (oy as f32) * grid.voxel_size.y,
                        grid.origin.z + (oz as f32) * grid.voxel_size.z,
                    );

                    corner_normals[i] = get_gradient(grid, ox, oy, oz);
                }

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];
                if edge_flags == 0 {
                    continue;
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
        }
    }

    Ok(Mesh { triangles })
}
