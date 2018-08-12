use std::marker::PhantomData;

use cache::{Cache, Insert};

pub(crate) struct CacheMetrics<K, V, C>
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
    pub(crate) fn with(cache: C) -> CacheMetrics<K, V, C> {
        CacheMetrics {
            cache,
            hits: 0,
            misses: 0,
            _key: PhantomData,
            _val: PhantomData,
        }
    }

    pub(crate) fn hits(&self) -> u64 {
        self.hits
    }

    pub(crate) fn misses(&self) -> u64 {
        self.misses
    }
}

impl<K: Clone, V, C: Cache<K, V>> Cache<K, V> for CacheMetrics<K, V, C> {
    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> Insert {
        use self::Insert::*;
        match self.cache.insert_if_missing(&key, creator) {
            AlreadyPresent => {
                self.hits += 1;
                AlreadyPresent
            }
            Inserted => {
                self.misses += 1;
                Inserted
            }
        }
    }
}
