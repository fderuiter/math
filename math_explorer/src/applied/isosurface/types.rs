use super::traits::ScalarField3D;

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
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        self.get(x, y, z)
    }

    fn voxel_size(&self) -> Point3D {
        self.voxel_size
    }

    fn origin(&self) -> Point3D {
        self.origin
    }
}
