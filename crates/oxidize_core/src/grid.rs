use std::ops::{Index, IndexMut};

/// A generic 2D grid container that standardizes coordinate-to-index flattening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D<T> {
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub height: usize,
    #[allow(missing_docs)]
    pub data: Vec<T>,
}

impl<T: Clone> Grid2D<T> {
    #[allow(missing_docs)]
    pub fn new(width: usize, height: usize, initial_value: T) -> Self {
        let size = width
            .checked_mul(height)
            .expect("Grid dimensions too large");
        Self {
            width,
            height,
            data: vec![initial_value; size],
        }
    }
}

impl<T> Grid2D<T> {
    #[allow(missing_docs)]
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            height,
            data,
        }
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn index_1d(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[self.index_1d(x, y)])
        } else {
            None
        }
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            let idx = self.index_1d(x, y);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid2D<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let idx = self.index_1d(x, y);
        &self.data[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid2D<T> {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = self.index_1d(x, y);
        &mut self.data[idx]
    }
}
