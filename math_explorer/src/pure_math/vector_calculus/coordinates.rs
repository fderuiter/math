use nalgebra::Vector3;

/// Trait for Orthogonal Coordinate Systems.
/// Defines how to convert to/from Cartesian and the scale factors.
pub trait OrthogonalCoordinateSystem {
    /// Converts a point from this coordinate system to Cartesian (x, y, z).
    fn local_to_cartesian(&self, point: &Vector3<f64>) -> Vector3<f64>;

    /// Converts a point from Cartesian (x, y, z) to this coordinate system.
    fn cartesian_to_local(&self, cartesian: &Vector3<f64>) -> Vector3<f64>;

    /// Returns the scale factors $(h_1, h_2, h_3)$ at the given point (in this coordinate system).
    fn scale_factors(&self, point: &Vector3<f64>) -> Vector3<f64>;
}

/// Cartesian Coordinates (x, y, z).
/// Scale factors are all 1.
pub struct Cartesian;

impl OrthogonalCoordinateSystem for Cartesian {
    fn local_to_cartesian(&self, point: &Vector3<f64>) -> Vector3<f64> {
        *point
    }

    fn cartesian_to_local(&self, cartesian: &Vector3<f64>) -> Vector3<f64> {
        *cartesian
    }

    fn scale_factors(&self, _point: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0, 1.0, 1.0)
    }
}

/// Cylindrical Coordinates $(\rho, \phi, z)$.
/// $x = \rho \cos\phi, y = \rho \sin\phi, z = z$.
pub struct Cylindrical;

impl OrthogonalCoordinateSystem for Cylindrical {
    fn local_to_cartesian(&self, point: &Vector3<f64>) -> Vector3<f64> {
        let rho = point[0];
        let phi = point[1];
        let z = point[2];
        Vector3::new(rho * phi.cos(), rho * phi.sin(), z)
    }

    fn cartesian_to_local(&self, cartesian: &Vector3<f64>) -> Vector3<f64> {
        let x = cartesian[0];
        let y = cartesian[1];
        let z = cartesian[2];
        let rho = (x * x + y * y).sqrt();
        let phi = y.atan2(x);
        Vector3::new(rho, phi, z)
    }

    fn scale_factors(&self, point: &Vector3<f64>) -> Vector3<f64> {
        let rho = point[0];
        Vector3::new(1.0, rho, 1.0)
    }
}

/// Spherical Coordinates $(r, \theta, \phi)$.
/// $x = r \sin\theta \cos\phi, y = r \sin\theta \sin\phi, z = r \cos\theta$.
/// $\theta$ is polar angle (from z-axis), $\phi$ is azimuthal angle (in xy-plane).
pub struct Spherical;

impl OrthogonalCoordinateSystem for Spherical {
    fn local_to_cartesian(&self, point: &Vector3<f64>) -> Vector3<f64> {
        let r = point[0];
        let theta = point[1];
        let phi = point[2];
        let sin_theta = theta.sin();
        Vector3::new(
            r * sin_theta * phi.cos(),
            r * sin_theta * phi.sin(),
            r * theta.cos(),
        )
    }

    fn cartesian_to_local(&self, cartesian: &Vector3<f64>) -> Vector3<f64> {
        let x = cartesian[0];
        let y = cartesian[1];
        let z = cartesian[2];
        let r = (x * x + y * y + z * z).sqrt();
        if r < 1e-12 {
            return Vector3::zeros();
        }
        let theta = (z / r).clamp(-1.0, 1.0).acos(); // acos returns [0, pi]
        let phi = y.atan2(x);
        Vector3::new(r, theta, phi)
    }

    fn scale_factors(&self, point: &Vector3<f64>) -> Vector3<f64> {
        let r = point[0];
        let theta = point[1];
        Vector3::new(1.0, r, r * theta.sin())
    }
}
