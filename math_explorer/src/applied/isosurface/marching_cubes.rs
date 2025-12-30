use super::tables::{CUBE_EDGE_FLAGS, EDGE_CONNECTION, TRIANGLE_CONNECTION_TABLE, VERTEX_OFFSET};
use super::traits::ScalarField3D;
use super::types::{Mesh, Point3D, Triangle};

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
fn get_gradient<F: ScalarField3D + ?Sized>(field: &F, x: usize, y: usize, z: usize) -> Point3D {
    let (width, height, depth) = field.dimensions();

    let dx = if x == 0 {
        field.value(x + 1, y, z) - field.value(x, y, z)
    } else if x == width - 1 {
        field.value(x, y, z) - field.value(x - 1, y, z)
    } else {
        (field.value(x + 1, y, z) - field.value(x - 1, y, z)) / 2.0
    };

    let dy = if y == 0 {
        field.value(x, y + 1, z) - field.value(x, y, z)
    } else if y == height - 1 {
        field.value(x, y, z) - field.value(x, y - 1, z)
    } else {
        (field.value(x, y + 1, z) - field.value(x, y - 1, z)) / 2.0
    };

    let dz = if z == 0 {
        field.value(x, y, z + 1) - field.value(x, y, z)
    } else if z == depth - 1 {
        field.value(x, y, z) - field.value(x, y, z - 1)
    } else {
        (field.value(x, y, z + 1) - field.value(x, y, z - 1)) / 2.0
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

    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 1e-6 {
        Point3D::new(nx / len, ny / len, nz / len)
    } else {
        Point3D::new(0.0, 0.0, 0.0)
    }
}

/// Helper struct to manage the state of a single cube processing
struct CubeProcessor<'a, F: ?Sized> {
    field: &'a F,
    threshold: f32,
    x: usize,
    y: usize,
    z: usize,
    corner_values: [f32; 8],
    corner_pos: [Point3D; 8],
    corner_normals: [Point3D; 8],
}

impl<'a, F: ScalarField3D + ?Sized> CubeProcessor<'a, F> {
    fn new(field: &'a F, threshold: f32, x: usize, y: usize, z: usize) -> Self {
        Self {
            field,
            threshold,
            x,
            y,
            z,
            corner_values: [0.0; 8],
            corner_pos: [Point3D::new(0.0, 0.0, 0.0); 8],
            corner_normals: [Point3D::new(0.0, 0.0, 0.0); 8],
        }
    }

    fn calculate_case_index(&mut self) -> usize {
        let mut cube_index = 0;
        let offsets = [
            (0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0),
            (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1),
        ];

        for (i, (dx, dy, dz)) in offsets.iter().enumerate() {
            let val = self.field.value(self.x + dx, self.y + dy, self.z + dz);
            self.corner_values[i] = val;
            if val < self.threshold {
                cube_index |= 1 << i;
            }
            // Defer position calculation until needed?
            // Actually we need them for interpolation later, so might as well compute them.
            // But if case is 0 or 255 we don't need them.
            // So we can lazily compute them if needed.
        }
        cube_index
    }

    fn compute_positions(&mut self) {
        let offsets = [
            (0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0),
            (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1),
        ];
        for (i, (dx, dy, dz)) in offsets.iter().enumerate() {
            self.corner_pos[i] = self.field.grid_to_world(self.x + dx, self.y + dy, self.z + dz);
        }
    }

    fn compute_gradients(&mut self) {
        for (i, offset) in VERTEX_OFFSET.iter().enumerate() {
            let ox = self.x + offset[0];
            let oy = self.y + offset[1];
            let oz = self.z + offset[2];
            self.corner_normals[i] = get_gradient(self.field, ox, oy, oz);
        }
    }

    fn interpolate_edges(&self, edge_flags: u16) -> ([Point3D; 12], [Point3D; 12]) {
        let mut edge_vertex = [Point3D::new(0.0, 0.0, 0.0); 12];
        let mut edge_norm = [Point3D::new(0.0, 0.0, 0.0); 12];

        for i in 0..12 {
            if (edge_flags & (1 << i)) != 0 {
                let v1_idx = EDGE_CONNECTION[i][0];
                let v2_idx = EDGE_CONNECTION[i][1];

                edge_vertex[i] = interpolate(
                    self.corner_pos[v1_idx], self.corner_values[v1_idx],
                    self.corner_pos[v2_idx], self.corner_values[v2_idx],
                    self.threshold,
                );

                edge_norm[i] = interpolate_normal(
                    self.corner_normals[v1_idx], self.corner_values[v1_idx],
                    self.corner_normals[v2_idx], self.corner_values[v2_idx],
                    self.threshold,
                );
            }
        }
        (edge_vertex, edge_norm)
    }
}

/// Extracts an isosurface from a 3D scalar field using the Marching Cubes algorithm.
///
/// This function is generic over any type that implements `ScalarField3D`, allowing it to work
/// with voxel grids, procedural generation, or other data sources.
pub fn extract_isosurface<F: ScalarField3D + ?Sized>(field: &F, threshold: f32) -> Result<Mesh, String> {
    let (width, height, depth) = field.dimensions();

    if width < 2 || height < 2 || depth < 2 {
        return Err("Grid dimensions must be at least 2x2x2".to_string());
    }

    // Estimate capacity to avoid reallocations
    // A heuristic: surface area roughly scales with N^2.
    // Let's reserve enough for a sphere of radius N/3.
    let estimated_triangles = width * height * 2;
    let mut triangles = Vec::with_capacity(estimated_triangles);

    // Iterate over each cube in the grid
    for z in 0..depth - 1 {
        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let mut processor = CubeProcessor::new(field, threshold, x, y, z);

                // 1. Determine the index of the case (0-255)
                let cube_index = processor.calculate_case_index();

                // 2. Check if the cube is entirely inside or outside
                let edge_flags = CUBE_EDGE_FLAGS[cube_index];
                if edge_flags == 0 {
                    continue;
                }

                // Now we know we need to process this cube, so we compute the heavy stuff
                processor.compute_positions();
                processor.compute_gradients();

                // 3. Compute intersection points on required edges
                let (edge_vertex, edge_norm) = processor.interpolate_edges(edge_flags);

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
