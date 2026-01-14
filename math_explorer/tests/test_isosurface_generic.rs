use math_explorer::applied::isosurface::{extract_isosurface, Volume, Point3D};

struct ProceduralSphere {
    radius: f32,
    dims: (usize, usize, usize),
    voxel_size: Point3D,
    origin: Point3D,
}

impl ProceduralSphere {
    fn new(radius: f32, size: usize) -> Self {
        let scale = (radius * 2.5) / (size as f32);
        Self {
            radius,
            dims: (size, size, size),
            voxel_size: Point3D::new(scale, scale, scale),
            origin: Point3D::new(-radius * 1.25, -radius * 1.25, -radius * 1.25),
        }
    }
}

impl Volume for ProceduralSphere {
    fn dimensions(&self) -> (usize, usize, usize) {
        self.dims
    }

    fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        let px = self.origin.x + (x as f32) * self.voxel_size.x;
        let py = self.origin.y + (y as f32) * self.voxel_size.y;
        let pz = self.origin.z + (z as f32) * self.voxel_size.z;

        // Signed Distance Function for a sphere: length(p) - r
        (px * px + py * py + pz * pz).sqrt() - self.radius
    }

    fn voxel_size(&self) -> Point3D {
        self.voxel_size
    }

    fn origin(&self) -> Point3D {
        self.origin
    }
}

#[test]
fn test_procedural_extraction() {
    let sphere = ProceduralSphere::new(10.0, 30);

    // Extract isosurface at 0.0 (surface of the sphere)
    let mesh = extract_isosurface(&sphere, 0.0).expect("Extraction failed");

    // Verify we got triangles
    assert!(mesh.triangles.len() > 0);

    // Verify bounds of the mesh match the sphere radius roughly
    for tri in mesh.triangles {
        for v in [tri.v1, tri.v2, tri.v3] {
            let dist = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
            // Vertices should be close to radius 10.0
            // Tolerance depends on resolution
            assert!((dist - 10.0).abs() < 1.0, "Vertex {:?} not on sphere surface", v);
        }
    }
}
