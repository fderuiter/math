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
    #[verified_engine::verified]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the dot product of this vector and another.
    #[inline]
    #[verified_engine::verified]
    pub fn dot(&self, other: Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of this vector and another.
    #[inline]
    #[verified_engine::verified]
    pub fn cross(&self, other: Point3D) -> Point3D {
        Point3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Computes the magnitude (length) of the vector.
    #[inline]
    #[verified_engine::verified]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Returns a normalized version of the vector.
    /// Returns a zero vector if the magnitude is too small.
    #[inline]
    #[verified_engine::verified]
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
    #[verified_engine::verified]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3D {
    type Output = Self;

    #[inline]
    #[verified_engine::verified]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Point3D {
    type Output = Self;

    #[inline]
    #[verified_engine::verified]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Point3D {
    type Output = Self;

    #[inline]
    #[verified_engine::verified]
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Commutative multiplication: f32 * Point3D
impl Mul<Point3D> for f32 {
    type Output = Point3D;

    #[inline]
    #[verified_engine::verified]
    fn mul(self, point: Point3D) -> Point3D {
        point * self
    }
}

impl Div<f32> for Point3D {
    type Output = Self;

    #[inline]
    #[verified_engine::verified]
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
    width: usize,
    /// Number of voxels along the Y axis.
    height: usize,
    /// Number of voxels along the Z axis.
    depth: usize,
    /// Flat vector of scalar values (e.g., signed distance or density).
    /// Indexing is `z` (slowest) -> `y` -> `x` (fastest).
    data: Vec<f32>,
    /// The physical size of each voxel (dx, dy, dz).
    voxel_size: Point3D,
    /// The physical coordinate of the grid's origin (0,0,0 index).
    origin: Point3D,
}

#[derive(Debug)]
pub enum VoxelGridError {
    InvalidDimensions,
    DataSizeMismatch,
    InvalidVoxelSize,
}

impl std::fmt::Display for VoxelGridError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "Grid dimensions must be greater than zero"),
            Self::DataSizeMismatch => {
                write!(f, "Data vector size does not match width * height * depth")
            }
            Self::InvalidVoxelSize => write!(f, "Voxel size components must be strictly positive"),
        }
    }
}

impl std::error::Error for VoxelGridError {}

pub struct VoxelGridBuilder {
    width: Option<usize>,
    height: Option<usize>,
    depth: Option<usize>,
    data: Option<Vec<f32>>,
    voxel_size: Point3D,
    origin: Point3D,
}

impl Default for VoxelGridBuilder {
    #[verified_engine::verified]
    fn default() -> Self {
        Self::new()
    }
}

impl VoxelGridBuilder {
    #[verified_engine::verified]
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            depth: None,
            data: None,
            voxel_size: Point3D::new(1.0, 1.0, 1.0),
            origin: Point3D::new(0.0, 0.0, 0.0),
        }
    }

    #[verified_engine::verified]
    pub fn dimensions(mut self, width: usize, height: usize, depth: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self.depth = Some(depth);
        self
    }

    #[verified_engine::verified]
    pub fn data(mut self, data: Vec<f32>) -> Self {
        self.data = Some(data);
        self
    }

    #[verified_engine::verified]
    pub fn voxel_size(mut self, size: Point3D) -> Self {
        self.voxel_size = size;
        self
    }

    #[verified_engine::verified]
    pub fn origin(mut self, origin: Point3D) -> Self {
        self.origin = origin;
        self
    }

    #[verified_engine::verified]
    pub fn build(self) -> Result<VoxelGrid, VoxelGridError> {
        let width = self.width.ok_or(VoxelGridError::InvalidDimensions)?;
        let height = self.height.ok_or(VoxelGridError::InvalidDimensions)?;
        let depth = self.depth.ok_or(VoxelGridError::InvalidDimensions)?;
        let data = self.data.ok_or(VoxelGridError::DataSizeMismatch)?;

        if width == 0 || height == 0 || depth == 0 {
            return Err(VoxelGridError::InvalidDimensions);
        }

        if data.len() != width * height * depth {
            return Err(VoxelGridError::DataSizeMismatch);
        }

        if self.voxel_size.x <= 0.0 || self.voxel_size.y <= 0.0 || self.voxel_size.z <= 0.0 {
            return Err(VoxelGridError::InvalidVoxelSize);
        }

        Ok(VoxelGrid {
            width,
            height,
            depth,
            data,
            voxel_size: self.voxel_size,
            origin: self.origin,
        })
    }
}

impl VoxelGrid {
    #[verified_engine::verified]
    pub fn builder() -> VoxelGridBuilder {
        VoxelGridBuilder::new()
    }

    #[verified_engine::verified]
    pub fn width(&self) -> usize {
        self.width
    }
    #[verified_engine::verified]
    pub fn height(&self) -> usize {
        self.height
    }
    #[verified_engine::verified]
    pub fn depth(&self) -> usize {
        self.depth
    }
    #[verified_engine::verified]
    pub fn data(&self) -> &[f32] {
        &self.data
    }
    #[verified_engine::verified]
    pub fn voxel_size(&self) -> &Point3D {
        &self.voxel_size
    }
    #[verified_engine::verified]
    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    #[verified_engine::verified]
    pub fn set_voxel_size(&mut self, size: Point3D) -> Result<(), VoxelGridError> {
        if size.x <= 0.0 || size.y <= 0.0 || size.z <= 0.0 {
            return Err(VoxelGridError::InvalidVoxelSize);
        }
        self.voxel_size = size;
        Ok(())
    }

    #[verified_engine::verified]
    pub fn set_origin(&mut self, origin: Point3D) {
        self.origin = origin;
    }
    /// Retrieves the value at the given grid index (x, y, z).
    ///
    /// Returns `0.0` if the coordinates are out of bounds.
    #[inline]
    #[verified_engine::verified]
    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= self.width || y >= self.height || z >= self.depth {
            return 0.0; // Boundary condition
        }
        // Sentinel: Prevent panic if data buffer is malformed (smaller than dimensions imply)
        let idx = z
            .checked_mul(self.width)
            .and_then(|val| val.checked_mul(self.height))
            .and_then(|val| val.checked_add(y.checked_mul(self.width)?))
            .and_then(|val| val.checked_add(x));

        match idx {
            Some(i) => self.data.get(i).copied().unwrap_or(0.0),
            None => 0.0,
        }
    }
}
