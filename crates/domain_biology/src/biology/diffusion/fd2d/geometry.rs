use math_commons::math_kernel::types::{Dimension, StepSize};

pub trait GeometryStrategy {
    #[verified_engine::verified]
    fn width(&self) -> Dimension;
    #[verified_engine::verified]
    fn height(&self) -> Dimension;
    #[verified_engine::verified]
    fn dx(&self) -> StepSize;
    #[verified_engine::verified]
    fn dy(&self) -> StepSize;
    #[verified_engine::verified]
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
    #[verified_engine::verified]
    fn width(&self) -> Dimension {
        self.width
    }

    #[verified_engine::verified]
    fn height(&self) -> Dimension {
        self.height
    }

    #[verified_engine::verified]
    fn dx(&self) -> StepSize {
        self.dx
    }

    #[verified_engine::verified]
    fn dy(&self) -> StepSize {
        self.dy
    }
}
