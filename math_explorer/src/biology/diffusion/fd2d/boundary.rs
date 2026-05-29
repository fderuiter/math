use crate::math_kernel::types::GridIndex;
use super::geometry::GeometryStrategy;

pub trait BoundaryStrategy {
    fn neighbors<G: GeometryStrategy>(&self, x: usize, y: usize, geom: &G) -> (GridIndex, GridIndex, GridIndex, GridIndex);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NeumannBoundary;

impl BoundaryStrategy for NeumannBoundary {
    #[inline(always)]
    fn neighbors<G: GeometryStrategy>(&self, x: usize, y: usize, geom: &G) -> (GridIndex, GridIndex, GridIndex, GridIndex) {
        let width = *geom.width();
        let height = *geom.height();
        
        let x_prev = if x > 0 { x - 1 } else { x };
        let x_next = if x < width - 1 { x + 1 } else { x };
        let y_prev = if y > 0 { y - 1 } else { y };
        let y_next = if y < height - 1 { y + 1 } else { y };

        let idx_l = y * width + x_prev;
        let idx_r = y * width + x_next;
        let idx_u = y_prev * width + x;
        let idx_d = y_next * width + x;

        (GridIndex(idx_l), GridIndex(idx_r), GridIndex(idx_u), GridIndex(idx_d))
    }
}
