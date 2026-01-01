use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE, VERTEX_OFFSET};
use super::types::{Mesh, Point3D, Triangle};
use super::traits::ScalarField3D;

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
fn get_gradient<S: ScalarField3D + ?Sized>(grid: &S, x: usize, y: usize, z: usize) -> Point3D {
    let dx = if x == 0 {
        grid.get(x + 1, y, z) - grid.get(x, y, z)
    } else if x == grid.width() - 1 {
        grid.get(x, y, z) - grid.get(x - 1, y, z)
    } else {
        (grid.get(x + 1, y, z) - grid.get(x - 1, y, z)) / 2.0
    };

    let dy = if y == 0 {
        grid.get(x, y + 1, z) - grid.get(x, y, z)
    } else if y == grid.height() - 1 {
        grid.get(x, y, z) - grid.get(x, y - 1, z)
    } else {
        (grid.get(x, y + 1, z) - grid.get(x, y - 1, z)) / 2.0
    };

    let dz = if z == 0 {
        grid.get(x, y, z + 1) - grid.get(x, y, z)
    } else if z == grid.depth() - 1 {
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


pub fn extract_isosurface<S: ScalarField3D + ?Sized>(grid: &S, threshold: f32) -> Result<Mesh, String> {
    if grid.width() < 2 || grid.height() < 2 || grid.depth() < 2 {
        return Err("Grid dimensions must be at least 2x2x2".to_string());
    }

    // Estimate capacity to avoid reallocations
    // A heuristic: surface area roughly scales with N^2.
    // Let's reserve enough for a sphere of radius N/3.
    let estimated_triangles = grid.width() * grid.height() * 2;
    let mut triangles = Vec::with_capacity(estimated_triangles);

    // stride_y and stride_z were used for direct indexing in Vec,
    // but now we abstract access via get().
    // Direct indexing optimization is traded for flexibility here.
    // The compiler might inline the get() call if the concrete type is known (monomorphization).

    let origin = grid.origin();
    let voxel_size = grid.voxel_size();

    // Iterate over each cube in the grid
    for z in 0..grid.depth() - 1 {
        let z_pos = origin.z + (z as f32) * voxel_size.z;

        for y in 0..grid.height() - 1 {
            let y_pos = origin.y + (y as f32) * voxel_size.y;

            for x in 0..grid.width() - 1 {
                let x_pos = origin.x + (x as f32) * voxel_size.x;

                // 1. Determine the index of the case (0-255)
                let mut cube_index = 0;
                let mut corner_values = [0.0; 8];
                let mut corner_pos = [Point3D::new(0.0,0.0,0.0); 8];
                // Profiler Note: Initializing normals here is wasteful if edge_flags == 0.
                // We will compute them lazily later.
                let mut corner_normals = [Point3D::new(0.0,0.0,0.0); 8];

                // Vertices are ordered:
                // 0: (0,0,0), 1: (1,0,0), 2: (1,1,0), 3: (0,1,0)
                // 4: (0,0,1), 5: (1,0,1), 6: (1,1,1), 7: (0,1,1)

                let v0 = grid.get(x, y, z);
                let v1 = grid.get(x + 1, y, z);
                let v2 = grid.get(x + 1, y + 1, z);
                let v3 = grid.get(x, y + 1, z);
                let v4 = grid.get(x, y, z + 1);
                let v5 = grid.get(x + 1, y, z + 1);
                let v6 = grid.get(x + 1, y + 1, z + 1);
                let v7 = grid.get(x, y + 1, z + 1);

                corner_values[0] = v0; if v0 < threshold { cube_index |= 1; }
                corner_values[1] = v1; if v1 < threshold { cube_index |= 2; }
                corner_values[2] = v2; if v2 < threshold { cube_index |= 4; }
                corner_values[3] = v3; if v3 < threshold { cube_index |= 8; }
                corner_values[4] = v4; if v4 < threshold { cube_index |= 16; }
                corner_values[5] = v5; if v5 < threshold { cube_index |= 32; }
                corner_values[6] = v6; if v6 < threshold { cube_index |= 64; }
                corner_values[7] = v7; if v7 < threshold { cube_index |= 128; }

                // Only compute positions if needed (though it's cheap)
                // We do it here to keep logic simple for step 3.
                let next_x_pos = x_pos + voxel_size.x;
                let next_y_pos = y_pos + voxel_size.y;
                let next_z_pos = z_pos + voxel_size.z;

                corner_pos[0] = Point3D::new(x_pos, y_pos, z_pos);
                corner_pos[1] = Point3D::new(next_x_pos, y_pos, z_pos);
                corner_pos[2] = Point3D::new(next_x_pos, next_y_pos, z_pos);
                corner_pos[3] = Point3D::new(x_pos, next_y_pos, z_pos);
                corner_pos[4] = Point3D::new(x_pos, y_pos, next_z_pos);
                corner_pos[5] = Point3D::new(next_x_pos, y_pos, next_z_pos);
                corner_pos[6] = Point3D::new(next_x_pos, next_y_pos, next_z_pos);
                corner_pos[7] = Point3D::new(x_pos, next_y_pos, next_z_pos);

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];
                if edge_flags == 0 {
                    continue;
                }

                // Profiler Optimization: Lazy Gradient Computation
                // Only compute gradients if the cube contains a surface intersection.
                for i in 0..8 {
                    let ox = x + VERTEX_OFFSET[i][0];
                    let oy = y + VERTEX_OFFSET[i][1];
                    let oz = z + VERTEX_OFFSET[i][2];
                    corner_normals[i] = get_gradient(grid, ox, oy, oz);
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
