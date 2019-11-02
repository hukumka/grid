use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem::swap;

pub struct CacheMap<K, V: ?Sized> {
    data: RefCell<HashMap<K, Box<V>>>,
    cache: RefCell<Cache<K, V>>,
}

struct Cache<K, V: ?Sized> {
    key: K,
    value: Option<Box<V>>,
}

impl<K: Default + Hash + Eq, V: ?Sized> CacheMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            cache: RefCell::new(Cache {
                key: K::default(),
                value: None,
            }),
        }
    }

    pub fn get_mut_or_insert<F: FnOnce() -> Box<V>>(&mut self, mut key: K, create: F) -> &mut V {
        let cache = self.cache.get_mut();
        let data = self.data.get_mut();
        if cache.key != key {
            let mut value = data.remove(&key);
            swap(&mut cache.key, &mut key);
            swap(&mut cache.value, &mut value);
            if let Some(value) = value {
                data.insert(key, value);
            }
        }
        if cache.value.is_none() {
            cache.value = Some(create());
        }
        cache.value.as_mut().unwrap().as_mut()
    }

    pub fn get(&self, mut key: K) -> Option<&V> {
        let mut cache = self.cache.borrow_mut();
        if cache.key != key {
            let mut data = self.data.borrow_mut();
            let mut value = data.remove(&key);
            swap(&mut cache.key, &mut key);
            swap(&mut cache.value, &mut value);
            if let Some(value) = value {
                data.insert(key, value);
            }
        }
        cache
            .value
            .as_ref()
            .map(|x| unsafe { &*(x.as_ref() as *const _) })
    }
}
