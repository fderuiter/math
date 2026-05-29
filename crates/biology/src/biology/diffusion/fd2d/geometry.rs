use math_core::math_kernel::types::{Dimension, StepSize};

pub trait GeometryStrategy {
    fn width(&self) -> Dimension;
    fn height(&self) -> Dimension;
    fn dx(&self) -> StepSize;
    fn dy(&self) -> StepSize;
    fn size(&self) -> usize {
        *self.width() * *self.height()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cartesian2D {
    pub width: Dimension,
    pub height: Dimension,
    pub dx: StepSize,
    pub dy: StepSize,
}

impl GeometryStrategy for Cartesian2D {
    fn width(&self) -> Dimension {
        self.width
    }

    fn height(&self) -> Dimension {
        self.height
    }

    fn dx(&self) -> StepSize {
        self.dx
    }

    fn dy(&self) -> StepSize {
        self.dy
    }
}
