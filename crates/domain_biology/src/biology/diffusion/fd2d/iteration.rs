use super::boundary::BoundaryStrategy;
use super::geometry::GeometryStrategy;
use math_commons::math_kernel::types::GridIndex;

pub trait IterationStrategy {
    fn iterate<G, B, F>(&self, geom: &G, boundary: &B, op: F)
    where
        G: GeometryStrategy,
        B: BoundaryStrategy,
        F: FnMut(GridIndex, GridIndex, GridIndex, GridIndex, GridIndex);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LoopSplittingIteration;

impl IterationStrategy for LoopSplittingIteration {
    #[inline(always)]
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn iterate<G, B, F>(&self, geom: &G, boundary: &B, mut op: F)
    where
        G: GeometryStrategy,
        B: BoundaryStrategy,
        F: FnMut(GridIndex, GridIndex, GridIndex, GridIndex, GridIndex),
    {
        let width = *geom.width();
        let height = *geom.height();

        // Fallback for small grids
        if width < 3 || height < 3 {
            for y in 0..height {
                for x in 0..width {
                    let idx = GridIndex(y * width + x);
                    let (idx_l, idx_r, idx_u, idx_d) = boundary.neighbors(x, y, geom);
                    op(idx, idx_l, idx_r, idx_u, idx_d);
                }
            }
            return;
        }

        // 1. Top Row (y=0)
        {
            let y = 0;
            for x in 0..width {
                let idx = GridIndex(x);
                let (idx_l, idx_r, idx_u, idx_d) = boundary.neighbors(x, y, geom);
                op(idx, idx_l, idx_r, idx_u, idx_d);
            }
        }

        // 2. Interior Rows
        for y in 1..height - 1 {
            let row_offset = y * width;

            // Left Col
            {
                let x = 0;
                let idx = GridIndex(row_offset);
                let (idx_l, idx_r, idx_u, idx_d) = boundary.neighbors(x, y, geom);
                op(idx, idx_l, idx_r, idx_u, idx_d);
            }

            // Interior (Fast path, no boundary checks)
            for x in 1..width - 1 {
                let idx = row_offset + x;
                let idx_l = idx - 1;
                let idx_r = idx + 1;
                let idx_u = idx - width;
                let idx_d = idx + width;
                op(
                    GridIndex(idx),
                    GridIndex(idx_l),
                    GridIndex(idx_r),
                    GridIndex(idx_u),
                    GridIndex(idx_d),
                );
            }

            // Right Col
            {
                let x = width - 1;
                let idx = GridIndex(row_offset + x);
                let (idx_l, idx_r, idx_u, idx_d) = boundary.neighbors(x, y, geom);
                op(idx, idx_l, idx_r, idx_u, idx_d);
            }
        }

        // 3. Bottom Row
        {
            let y = height - 1;
            for x in 0..width {
                let idx = GridIndex(y * width + x);
                let (idx_l, idx_r, idx_u, idx_d) = boundary.neighbors(x, y, geom);
                op(idx, idx_l, idx_r, idx_u, idx_d);
            }
        }
    }
}
