use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct FenwickTree2D<T> {
    value: Arr2d<T>,
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree2D<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            value: Arr2d::new(rows, cols, T::zero()),
        }
    }

    #[inline]
    pub fn get_to(&self, mut row: usize, col: usize) -> T {
        let mut result = T::zero();
        while row > 0 {
            let mut c = col;
            while c > 0 {
                result += self.value[row - 1][c - 1];
                c &= c - 1;
            }
            row &= row - 1;
        }
        result
    }

    pub fn get(
        &self,
        row_bounds: impl RangeBounds<usize>,
        col_bounds: impl RangeBounds<usize>,
    ) -> T {
        let row_from = match row_bounds.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let row_to = match row_bounds.end_bound() {
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Unbounded => self.value.d1(),
        };
        let col_from = match col_bounds.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let col_to = match col_bounds.end_bound() {
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Unbounded => self.value.d2(),
        };
        if row_from >= row_to || col_from >= col_to {
            T::zero()
        } else {
            self.get_to(row_to, col_to)
                - self.get_to(row_from, col_to)
                - self.get_to(row_to, col_from)
                + self.get_to(row_from, col_from)
        }
    }

    pub fn add(&mut self, mut row: usize, col: usize, v: T) {
        while row < self.value.d1() {
            let mut c = col;
            while c < self.value[row].len() {
                self.value[row][c] += v;
                c |= c + 1;
            }
            row |= row + 1;
        }
    }
}

impl<T: AdditionMonoidWithSub + Copy> From<&[Vec<T>]> for FenwickTree2D<T> {
    fn from(grid: &[Vec<T>]) -> Self {
        let mut result = Self::new(grid.len(), grid[0].len());
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                result.add(i, j, v);
            }
        }
        result
    }
}
