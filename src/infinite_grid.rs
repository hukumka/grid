use crate::auto_index_impl::{IndexRefContainer, IndexRefContainerMut};
use crate::{
    auto_index_impl::{ImplIndexForRef, IndexForRef},
    cache_map::CacheMap,
    slice::GridSlice,
};
use std::ops;
use std::ops::Range;
use crate::slice::FromGridSlice;

const CHUNK_SIDE_LG2: i32 = 6;
const CHUNK_SIDE: i32 = 1 << CHUNK_SIDE_LG2;
const CHUNK_SIDE_MASK: i32 = (1 << CHUNK_SIDE_LG2) - 1;

pub struct InfiniteGrid<T> {
    data: CacheMap<(i32, i32), [T]>,
    default: T,
}

pub type InfiniteGridSliceRef<'a, T> = GridSlice<IndexRefContainer<'a, InfiniteGrid<T>>, i32>;
pub type InfiniteGridSliceMut<'a, T> = GridSlice<IndexRefContainerMut<'a, InfiniteGrid<T>>, i32>;

impl<T: Default> Default for InfiniteGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> InfiniteGrid<T> {
    pub fn new() -> Self {
        Self {
            data: CacheMap::new(),
            default: T::default(),
        }
    }

    pub fn slice(&self, x: Range<i32>, y: Range<i32>) -> InfiniteGridSliceRef<'_, T> {
        GridSlice::new(IndexForRef::new(self), x, y)
    }

    pub fn slice_mut(&mut self, x: Range<i32>, y: Range<i32>) -> InfiniteGridSliceMut<'_, T> {
        GridSlice::new(IndexForRef::new(self), x, y)
    }

    fn chunk_offset((x, y): (i32, i32)) -> usize {
        debug_assert!(0 <= x && x < CHUNK_SIDE);
        debug_assert!(0 <= y && y < CHUNK_SIDE);
        (x + y * CHUNK_SIDE) as usize
    }

    fn split_coords((x, y): (i32, i32)) -> ((i32, i32), (i32, i32)) {
        (
            (x & CHUNK_SIDE_MASK, y & CHUNK_SIDE_MASK),
            (x >> CHUNK_SIDE_LG2, y >> CHUNK_SIDE_LG2),
        )
    }
}

impl<T: Default> ops::Index<(i32, i32)> for InfiniteGrid<T> {
    type Output = T;
    fn index(&self, key: (i32, i32)) -> &T {
        let (pos, chunk_id) = Self::split_coords(key);
        let offset = Self::chunk_offset(pos);
        self.data
            .get(chunk_id)
            .map(|x| &x[offset])
            .unwrap_or(&self.default)
    }
}

impl<T: Default> ops::IndexMut<(i32, i32)> for InfiniteGrid<T> {
    fn index_mut(&mut self, key: (i32, i32)) -> &mut T {
        let (pos, chunk_id) = Self::split_coords(key);
        let chunk = self.data.get_mut_or_insert(chunk_id, || {
            let size = (CHUNK_SIDE * CHUNK_SIDE) as usize;
            let mut data = Vec::with_capacity(size);
            for _ in 0..size {
                data.push(T::default());
            }
            data.into_boxed_slice()
        });
        &mut chunk[Self::chunk_offset(pos)]
    }
}

impl<T: Default> ImplIndexForRef<(i32, i32)> for InfiniteGrid<T> {}

impl<I: Clone + Default> FromGridSlice<i32, I> for InfiniteGrid<I> {
    fn from_slice<T>(slice: &GridSlice<T, i32>) -> Self
    where
        T: ops::Index<(i32, i32), Output=I>,
    {
        let xr = slice.x_range();
        let yr = slice.y_range();
        let w = xr.end - xr.start;
        let h = yr.start - yr.end;
        let mut res = Self::new();
        for y in 0..h {
            for x in 0..w {
                res[(x, y)] = slice[(x + xr.start, y + yr.start)].clone()
            }
        }
        res
    }
}
