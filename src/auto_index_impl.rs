use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct IndexForRef<C, T> {
    container: C,
    _marker: PhantomData<T>,
}

impl<C, T> IndexForRef<C, T> {
    pub fn new(x: C) -> Self {
        Self {
            container: x,
            _marker: Default::default(),
        }
    }
}

impl<T, C: Borrow<T>> IndexForRef<C, T> {
    pub fn get(&self) -> &T {
        self.container.borrow()
    }
}

impl<T, C: BorrowMut<T>> IndexForRef<C, T> {
    pub fn get_mut(&mut self) -> &mut T {
        self.container.borrow_mut()
    }
}

pub type IndexRefContainer<'a, T> = IndexForRef<&'a T, T>;
pub type IndexRefContainerMut<'a, T> = IndexForRef<&'a mut T, T>;

pub trait ImplIndexForRef<K>: Index<K> + IndexMut<K> {}

impl<K, C, T> Index<K> for IndexForRef<C, T>
where
    T: ImplIndexForRef<K>,
    C: Borrow<T>,
{
    type Output = <T as Index<K>>::Output;
    fn index(&self, key: K) -> &Self::Output {
        self.container.borrow().index(key)
    }
}

impl<K, C, T> IndexMut<K> for IndexForRef<C, T>
where
    T: ImplIndexForRef<K>,
    C: BorrowMut<T>,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.container.borrow_mut().index_mut(key)
    }
}
