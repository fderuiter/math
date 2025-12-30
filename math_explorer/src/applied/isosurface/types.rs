#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Point3D,
    pub v2: Point3D,
    pub v3: Point3D,
    pub n1: Point3D,
    pub n2: Point3D,
    pub n3: Point3D,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

#[derive(Debug, Clone)]
pub struct VoxelGrid {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub data: Vec<f32>, // Flat vector, row-major or z-major
    pub voxel_size: Point3D,
    pub origin: Point3D,
}

use super::traits::ScalarField3D;

impl VoxelGrid {
    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= self.width || y >= self.height || z >= self.depth {
            return 0.0; // Boundary condition
        }
        self.data[z * self.width * self.height + y * self.width + x]
    }
}

impl ScalarField3D for VoxelGrid {
    #[inline]
    fn value(&self, x: usize, y: usize, z: usize) -> f32 {
        self.get(x, y, z)
    }

    #[inline]
    fn dimensions(&self) -> (usize, usize, usize) {
        (self.width, self.height, self.depth)
    }

    #[inline]
    fn grid_to_world(&self, x: usize, y: usize, z: usize) -> Point3D {
        Point3D::new(
            self.origin.x + (x as f32) * self.voxel_size.x,
            self.origin.y + (y as f32) * self.voxel_size.y,
            self.origin.z + (z as f32) * self.voxel_size.z,
        )
    }
}
