use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3D point or vector with single-precision floating point coordinates.
///
/// Implements standard arithmetic operations (Add, Sub, Mul/Div by scalar) to facilitate
/// vector mathematics.
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
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns a zero vector (0, 0, 0).
    #[inline]
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculates the dot product with another vector.
    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product with another vector.
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the squared magnitude (length) of the vector.
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Returns a normalized unit vector. Returns zero vector if magnitude is close to zero.
    #[inline]
    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m > 1e-6 {
            *self / m
        } else {
            Self::zero()
        }
    }
}

// Vector Addition
impl Add for Point3D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Point3D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vector Subtraction
impl Sub for Point3D {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Point3D {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Scalar Multiplication
impl Mul<f32> for Point3D {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f32> for Point3D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// Commutative Scalar Multiplication (f32 * Point3D) requires a wrapper or is not possible directly on foreign types (f32).
// We only implement Point3D * f32.

// Scalar Division
impl Div<f32> for Point3D {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self {
        let inv = 1.0 / scalar;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl DivAssign<f32> for Point3D {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        let inv = 1.0 / scalar;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

// Negation
impl Neg for Point3D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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
