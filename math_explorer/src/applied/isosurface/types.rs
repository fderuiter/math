use std::ops::{Add, Div, Mul, Neg, Sub};

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

    /// Returns a zero vector (0, 0, 0).
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Calculates the dot product with another vector.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product with another vector.
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Calculates the magnitude (length) of the vector.
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the squared magnitude of the vector (faster than magnitude).
    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns a normalized unit vector. Returns zero vector if magnitude is close to zero.
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag > 1e-6 {
            self / mag
        } else {
            Self::zero()
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Point3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Allow commutative multiplication (f32 * Point3D) - sadly Rust doesn't allow implementing foreign traits on foreign types easily without macros or newtypes in the crate defining the trait, but we can't do `impl Mul<Point3D> for f32` here.
// So we just stick to Point3D * f32.

impl Div<f32> for Point3D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        // Multiplication by reciprocal is faster but less precise.
        // Keeping it safe with division as per memory "Point3D::div uses direct division"
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Point3D {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
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
