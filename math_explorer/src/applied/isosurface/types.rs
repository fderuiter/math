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
    /// Public constant for epsilon comparisons.
    pub const EPSILON: f32 = 1.0e-6;

    /// Creates a new `Point3D` with the given coordinates.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the dot product of this vector and another.
    #[inline]
    pub fn dot(&self, other: Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of this vector and another.
    #[inline]
    pub fn cross(&self, other: Point3D) -> Point3D {
        Point3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Computes the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Returns a normalized version of the vector.
    /// Returns a zero vector if the magnitude is too small.
    #[inline]
    pub fn normalize(&self) -> Point3D {
        let mag = self.magnitude();
        if mag > Self::EPSILON {
            *self / mag
        } else {
            Point3D::new(0.0, 0.0, 0.0)
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Point3D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Point3D {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Commutative multiplication: f32 * Point3D
impl Mul<Point3D> for f32 {
    type Output = Point3D;

    #[inline]
    fn mul(self, point: Point3D) -> Point3D {
        point * self
    }
}

impl Div<f32> for Point3D {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self {
        // Optimization: multiply by reciprocal if scalar is constant-ish,
        // but here we just divide.
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
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
        // Sentinel: Prevent panic if data buffer is malformed (smaller than dimensions imply)
        let idx = z * self.width * self.height + y * self.width + x;
        self.data.get(idx).copied().unwrap_or(0.0)
    }
}
