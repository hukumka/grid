use crate::auto_index_impl::{IndexRefContainer, IndexRefContainerMut};
use crate::slice::FromGridSlice;
use crate::{
    auto_index_impl::{ImplIndexForRef, IndexForRef},
    slice::GridSlice,
};
use std::fmt;
use std::ops;
use std::ops::RangeBounds;

/// 2d grid of fixed size
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    dims: (usize, usize),
}

impl<T> fmt::Debug for Grid<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

fn flat_offset((x, y): (usize, usize), (w, h): (usize, usize)) -> usize {
    debug_assert!(x < w);
    debug_assert!(y < h);
    x + y * w
}

impl<T: Default> Grid<T> {
    /// Create new instance of grid filled with default elements
    pub fn new_default((width, height): (usize, usize)) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(T::default());
        }
        Grid { data, dims: (width, height) }
    }
}

impl<T: Clone> Grid<T>{
    /// Create new instance of grid filled with default elements
    pub fn new((width, height): (usize, usize), fill_with: T) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(fill_with.clone());
        }
        Grid { data, dims: (width, height) }
    }
}

impl<T> ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, key: (usize, usize)) -> &Self::Output {
        &self.data[flat_offset(key, self.dims)]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, key: (usize, usize)) -> &mut Self::Output {
        &mut self.data[flat_offset(key, self.dims)]
    }
}

impl<T> ImplIndexForRef<(usize, usize)> for Grid<T> {}

pub type GridSliceRef<'a, T> = GridSlice<IndexRefContainer<'a, Grid<T>>, usize>;
pub type GridSliceMut<'a, T> = GridSlice<IndexRefContainerMut<'a, Grid<T>>, usize>;

impl<T> Grid<T> {
    /// Get grid shape
    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }

    /// Get immutable view of square part of grid
    ///
    /// ```
    /// use grid::Grid;
    /// let mut grid = Grid::new(10, 10);
    /// grid[(1, 2)] = 3;
    /// grid[(2, 2)] = 1;
    /// assert_eq!(grid.slice(.., 1..)[(1, 1)], 3);
    ///
    /// grid.slice(1..3, 1..3).iter()
    ///     .zip(&[
    ///         0, 0,
    ///         3, 1
    ///     ])
    ///     .inspect(|(a, b)| assert_eq!(a, b))
    ///     .count();
    /// ```
    pub fn slice<K1, K2>(&self, x: K1, y: K2) -> GridSliceRef<T>
    where
        K1: RangeBounds<usize>,
        K2: RangeBounds<usize>,
    {
        let (x1, x2) = Self::range_bound_to_pair(x, self.dims.0);
        let (y1, y2) = Self::range_bound_to_pair(y, self.dims.1);
        GridSlice::new(IndexForRef::new(self), x1..x2, y1..y2)
    }

    /// Get mutable view of square part of grid
    ///
    /// ```
    /// use grid::Grid;
    /// let mut grid = Grid::new(10, 10);
    /// grid[(1, 2)] = 3;
    /// grid[(2, 2)] = 1;
    ///
    /// let mut grid2 = Grid::new(5, 5);
    /// grid2.slice_mut(0..2, 0..2).clone_from(&grid.slice(1..3, 1..3));
    ///
    /// grid2.slice(0..2, 0..2).iter()
    ///     .zip(&[
    ///         0, 0,
    ///         3, 1
    ///     ])
    ///     .inspect(|(a, b)| assert_eq!(a, b))
    ///     .count();
    /// ```
    pub fn slice_mut<K1, K2>(&mut self, x: K1, y: K2) -> GridSliceMut<T>
    where
        K1: RangeBounds<usize>,
        K2: RangeBounds<usize>,
    {
        let (x1, x2) = Self::range_bound_to_pair(x, self.dims.0);
        let (y1, y2) = Self::range_bound_to_pair(y, self.dims.1);
        GridSlice::new(IndexForRef::new(self), x1..x2, y1..y2)
    }

    fn range_bound_to_pair<K: RangeBounds<usize>>(bound: K, len: usize) -> (usize, usize) {
        use std::ops::Bound::*;
        let end = match bound.end_bound() {
            Included(x) => *x + 1,
            Excluded(x) => *x,
            Unbounded => len,
        };
        debug_assert!(end <= len, "{} !<= {}", end, len);
        let start = match bound.start_bound() {
            Included(x) => *x,
            Excluded(x) => *x + 1,
            Unbounded => 0,
        };
        (start, end)
    }

    /// Create immutable slice over all grid
    ///
    /// same as
    /// `.slice(.., ..)`
    pub fn as_slice(&self) -> GridSliceRef<T> {
        GridSlice::new(IndexForRef::new(self), 0..self.dims.0, 0..self.dims.1)
    }

    /// Create mutable slice over all grid
    ///
    /// same as
    /// `.slice_mut(.., ..)`
    pub fn as_slice_mut(&mut self) -> GridSliceMut<T> {
        let x = 0..self.dims.0;
        let y = 0..self.dims.1;
        GridSlice::new(IndexForRef::new(self), x, y)
    }
}

impl<I: Clone> FromGridSlice<usize, I> for Grid<I> {
    fn from_slice<T>(slice: &GridSlice<T, usize>) -> Self
    where
        T: ops::Index<(usize, usize), Output=I>,
    {
        let xr = slice.x_range();
        let yr = slice.y_range();
        let w = xr.end - xr.start;
        let h = yr.start - yr.end;
        let mut data = Vec::with_capacity(w * h);
        for y in yr {
            for x in xr.clone() {
                data.push(slice[(x, y)].clone());
            }
        }
        Self { data, dims: (w, h) }
    }
}
