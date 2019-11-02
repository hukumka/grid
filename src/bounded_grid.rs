use crate::auto_index_impl::{IndexRefContainer, IndexRefContainerMut};
use crate::{
    auto_index_impl::{ImplIndexForRef, IndexForRef},
    slice::GridSlice,
};
use std::fmt;
use std::ops;
use std::ops::RangeBounds;

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
    pub fn new(w: usize, h: usize) -> Self {
        let mut data = Vec::with_capacity(w * h);
        for _ in 0..(w * h) {
            data.push(T::default());
        }
        Grid { data, dims: (w, h) }
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
    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }

    pub fn slice<K1, K2>(&self, x: K1, y: K2) -> GridSliceRef<T>
    where
        K1: RangeBounds<usize>,
        K2: RangeBounds<usize>,
    {
        let (x1, x2) = Self::range_bound_to_pair(x, self.dims.0);
        let (y1, y2) = Self::range_bound_to_pair(y, self.dims.1);
        GridSlice::new(IndexForRef::new(self), x1..x2, y1..y2)
    }

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

    pub fn as_slice(&self) -> GridSliceRef<T> {
        GridSlice::new(IndexForRef::new(self), 0..self.dims.0, 0..self.dims.1)
    }

    pub fn as_slice_mut(&mut self) -> GridSliceMut<T> {
        let x = 0..self.dims.0;
        let y = 0..self.dims.1;
        GridSlice::new(IndexForRef::new(self), x, y)
    }
}
