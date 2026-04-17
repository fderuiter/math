#![allow(clippy::all)]
#![allow(warnings)]
#[cfg(test)]
mod tests {
    use math_explorer::applied::isosurface::{Point3D, VoxelGrid, extract_isosurface};

    #[test]
    fn test_sphere_extraction() {
        // Create a 20x20x20 grid
        let size = 20;
        let mut data = vec![0.0; size * size * size];
        let center = size as f32 / 2.0;
        let radius = 6.0;

        // Fill with signed distance field of a sphere (or just distance)
        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    let dz = z as f32 - center;
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                    data[z * size * size + y * size + x] = dist;
                }
            }
        }

        let grid = VoxelGrid {
            width: size,
            height: size,
            depth: size,
            data,
            voxel_size: Point3D::new(1.0, 1.0, 1.0),
            origin: Point3D::new(0.0, 0.0, 0.0),
        };

        // Extract at radius 6.0
        let mesh = extract_isosurface(&grid, radius).unwrap();

        // A sphere should have many triangles
        assert!(mesh.triangles.len() > 100);

        // Check if vertices are approximately at radius distance from center
        for tri in &mesh.triangles {
            for v in &[tri.v1, tri.v2, tri.v3] {
                let dx = v.x - center;
                let dy = v.y - center;
                let dz = v.z - center;
                let d = (dx * dx + dy * dy + dz * dz).sqrt();
                // We used linear interpolation, so it should be close to exact radius
                // Increase tolerance slightly due to discretization
                assert!(
                    (d - radius).abs() < 1.0,
                    "Vertex distance {} not close to radius {}",
                    d,
                    radius
                );
            }
        }
    }

    #[test]
    fn test_small_grid() {
        let grid = VoxelGrid {
            width: 2,
            height: 2,
            depth: 2,
            data: vec![0.0, 10.0, 0.0, 10.0, 0.0, 10.0, 0.0, 10.0], // Alternating values
            voxel_size: Point3D::new(1.0, 1.0, 1.0),
            origin: Point3D::new(0.0, 0.0, 0.0),
        };
        let mesh = extract_isosurface(&grid, 5.0).unwrap();
        assert!(mesh.triangles.len() > 0);
    }
}
