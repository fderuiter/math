/// A 3D point or vector with single-precision floating point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
}

impl Point3D {
    /// Creates a new `Point3D` with the given coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// A triangle defined by three vertices and three vertex normals.
#[derive(Debug, Clone)]
pub struct Triangle {
    /// First vertex position
    pub v1: Point3D,
    /// Second vertex position
    pub v2: Point3D,
    /// Third vertex position
    pub v3: Point3D,
    /// Normal at the first vertex
    pub n1: Point3D,
    /// Normal at the second vertex
    pub n2: Point3D,
    /// Normal at the third vertex
    pub n3: Point3D,
}

/// A collection of triangles representing a 3D surface.
#[derive(Debug, Clone)]
pub struct Mesh {
    /// The list of triangles that make up the mesh.
    pub triangles: Vec<Triangle>,
}

/// A 3D grid of scalar values, used as input for isosurface extraction.
///
/// The grid data is stored in a flat vector, indexed by `z * width * height + y * width + x`.
#[derive(Debug, Clone)]
pub struct VoxelGrid {
    /// Number of voxels along the X axis.
    pub width: usize,
    /// Number of voxels along the Y axis.
    pub height: usize,
    /// Number of voxels along the Z axis.
    pub depth: usize,
    /// Flat vector of scalar values (e.g., signed distance or density).
    /// Indexing is `z` (slowest) -> `y` -> `x` (fastest).
    pub data: Vec<f32>,
    /// The physical size of each voxel (dx, dy, dz).
    pub voxel_size: Point3D,
    /// The physical coordinate of the grid's origin (0,0,0 index).
    pub origin: Point3D,
}

impl VoxelGrid {
    /// Retrieves the value at the given grid index (x, y, z).
    ///
    /// Returns `0.0` if the coordinates are out of bounds.
    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= self.width || y >= self.height || z >= self.depth {
            return 0.0; // Boundary condition
        }
        self.data[z * self.width * self.height + y * self.width + x]
    }
}
