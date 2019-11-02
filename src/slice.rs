use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::Step;
use std::ops;
use std::ops::Range;

pub trait FromGridSlice<K: Step + Copy> {
    fn from_slice<T>(slice: &GridSlice<T, K>) -> Self
    where
        T: ops::Index<(K, K)>,
        K: Step + Copy,
        <T as ops::Index<(K, K)>>::Output: Clone;
}

#[derive(Clone)]
pub struct GridSlice<T, K> {
    grid: T,
    x: Range<K>,
    y: Range<K>,
}

impl<T, K> GridSlice<T, K> {
    pub fn new(grid: T, x: Range<K>, y: Range<K>) -> Self {
        Self { grid, x, y }
    }
}

impl<T, K> fmt::Debug for GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    <T as ops::Index<(K, K)>>::Output: fmt::Debug,
    K: Step + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid<?>{{")?;
        for i in self.y.clone() {
            let mut iter = self.x.clone();
            let zero = iter.next().unwrap();
            write!(f, "{:?}", self.grid.index((zero, i)))?;
            for j in iter {
                write!(f, ", {:?}", self.grid.index((j, i)))?;
            }
            writeln!(f)?;
        }
        writeln!(f, "}}")
    }
}

impl<T, K> Eq for GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    <T as ops::Index<(K, K)>>::Output: PartialEq,
    K: Step + Copy + Eq + ops::Sub<K, Output = K>,
{
}

impl<T, K> PartialEq for GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    <T as ops::Index<(K, K)>>::Output: PartialEq,
    K: Step + Copy + PartialEq + ops::Sub<K, Output = K>,
{
    fn eq(&self, other: &Self) -> bool {
        let get_len = |range: &Range<K>| range.end - range.start;
        if get_len(&self.x) != get_len(&other.x) || get_len(&self.y) != get_len(&other.y) {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T, K> std::hash::Hash for GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    <T as ops::Index<(K, K)>>::Output: Hash,
    K: Step + Copy + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.iter() {
            i.hash(state);
        }
    }
}

impl<T, K> GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    K: Step + Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = &<T as ops::Index<(K, K)>>::Output> {
        self.x
            .clone()
            .flat_map(move |x| self.y.clone().map(move |y| (x, y)))
            .map(move |key| self.grid.index(key))
    }

    pub fn iter_indices(
        &self,
    ) -> impl Iterator<Item = ((K, K), &<T as ops::Index<(K, K)>>::Output)> {
        let y = self.y.clone();
        self.x
            .clone()
            .flat_map(move |x| y.clone().map(move |y| (x, y)))
            .map(move |key| (key, self.grid.index(key)))
    }
}

impl<T, K> GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    K: Step + Copy,
    <T as ops::Index<(K, K)>>::Output: Clone,
{
    pub fn clone_into<X: FromGridSlice<K>>(&self) -> X {
        X::from_slice(&self)
    }
}

impl<T, K> GridSlice<T, K>
where
    T: ops::IndexMut<(K, K)>,
    K: Step + PartialEq + Copy,
    <T as ops::Index<(K, K)>>::Output: Clone,
{
    pub fn clone_from<T2>(&mut self, source: &GridSlice<T2, K>)
    where
        T2: ops::Index<(K, K), Output = <T as ops::Index<(K, K)>>::Output>,
    {
        for (y1, y2) in self.y.clone().zip(source.y.clone()) {
            for (x1, x2) in self.x.clone().zip(source.x.clone()) {
                *self.grid.index_mut((x1, y1)) = source.grid.index((x2, y2)).clone();
            }
        }
    }
}

impl<T, K> ops::Index<(K, K)> for GridSlice<T, K>
where
    T: ops::Index<(K, K)>,
    K: Copy + ops::Add<K, Output = K>,
{
    type Output = <T as ops::Index<(K, K)>>::Output;
    fn index(&self, (x, y): (K, K)) -> &Self::Output {
        self.grid.index((self.x.start + x, self.y.start + y))
    }
}

impl<T, K> ops::IndexMut<(K, K)> for GridSlice<T, K>
where
    T: ops::IndexMut<(K, K)>,
    K: Copy + ops::Add<K, Output = K>,
{
    fn index_mut(&mut self, (x, y): (K, K)) -> &mut <T as ops::Index<(K, K)>>::Output {
        self.grid.index_mut((self.x.start + x, self.y.start + y))
    }
}
