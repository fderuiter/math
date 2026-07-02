pub trait BoundaryCondition {
    /// Given a coordinate (x, y) and grid dimensions (width, height),
    /// resolve the target index based on boundary logic.
    fn resolve(&self, x: isize, y: isize, width: usize, height: usize) -> Option<usize>;
}

pub struct PeriodicBoundary;
impl BoundaryCondition for PeriodicBoundary {
    fn resolve(&self, x: isize, y: isize, width: usize, height: usize) -> Option<usize> {
        let x_mod = x.rem_euclid(width as isize) as usize;
        let y_mod = y.rem_euclid(height as isize) as usize;
        Some(y_mod * width + x_mod)
    }
}

pub struct BounceBackBoundary;
impl BoundaryCondition for BounceBackBoundary {
    fn resolve(&self, x: isize, y: isize, width: usize, height: usize) -> Option<usize> {
        if x < 0 || y < 0 || x >= width as isize || y >= height as isize {
            None // Signifies an out-of-bounds that requires bounce-back
        } else {
            Some((y as usize) * width + (x as usize))
        }
    }
}

pub struct NeumannBoundary;
impl BoundaryCondition for NeumannBoundary {
    fn resolve(&self, x: isize, y: isize, width: usize, height: usize) -> Option<usize> {
        let mut x_clamped = x;
        let mut y_clamped = y;

        if x_clamped < 0 {
            x_clamped = 0;
        }
        if x_clamped >= width as isize {
            x_clamped = width as isize - 1;
        }
        if y_clamped < 0 {
            y_clamped = 0;
        }
        if y_clamped >= height as isize {
            y_clamped = height as isize - 1;
        }

        Some((y_clamped as usize) * width + (x_clamped as usize))
    }
}
