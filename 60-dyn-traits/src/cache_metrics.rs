use std::marker::PhantomData;

use cache::{Cache, Insert};

pub struct CacheMetrics<K, V, C>
where
    C: Cache<K, V>,
    K: Clone,
{
    cache: C,
    hits: u64,
    misses: u64,
    _key: PhantomData<K>,
    _val: PhantomData<V>,
}

impl<K: Clone, V, C: Cache<K, V>> CacheMetrics<K, V, C> {
    pub fn with(cache: C) -> CacheMetrics<K, V, C> {
        CacheMetrics {
            cache,
            hits: 0,
            misses: 0,
            _key: PhantomData,
            _val: PhantomData,
        }
    }

    pub fn hits(&self) -> u64 {
        self.hits
    }

    pub fn misses(&self) -> u64 {
        self.misses
    }
}

impl<K: Clone, V, C: Cache<K, V>> Cache<K, V> for CacheMetrics<K, V, C> {
    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn insert(&mut self, key: K, val: V) {
        self.cache.insert(key, val);
    }

    fn insert_if_missing(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Insert {
        self.cache.insert_if_missing(&key, creator)
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        match self.insert_if_missing(&key, creator) {
            Insert::AlreadyPresent => self.hits += 1,
            Insert::Inserted => self.misses += 1,
        }
        self.get(&key).unwrap()
    }
}
