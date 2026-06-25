use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 3D point or vector with single-precision floating point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub const EPSILON: f32 = 1.0e-6;
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn dot(&self, other: Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline]
    pub fn cross(&self, other: Point3D) -> Point3D {
        Point3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }
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

impl Add for Point3D { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) } }
impl Sub for Point3D { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) } }
impl Neg for Point3D { type Output = Self; #[inline] fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) } }
impl Mul<f32> for Point3D { type Output = Self; #[inline] fn mul(self, scalar: f32) -> Self { Self::new(self.x * scalar, self.y * scalar, self.z * scalar) } }
impl Mul<Point3D> for f32 { type Output = Point3D; #[inline] fn mul(self, point: Point3D) -> Point3D { point * self } }
impl Div<f32> for Point3D { type Output = Self; #[inline] fn div(self, scalar: f32) -> Self { Self::new(self.x / scalar, self.y / scalar, self.z / scalar) } }

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
