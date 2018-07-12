#![feature(rust_2018_preview, nll)]

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub enum CacheInsert {
        AlreadyPresent,
        Inserted,
    }

    pub struct Cache<K: Hash + Eq, V> {
        map: HashMap<K, V>,
    }

    impl<K: Hash + Eq + Clone, V> Cache<K, V> {
        pub fn new() -> Cache<K, V> {
            Cache {
                map: HashMap::new(),
            }
        }

        pub fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            self.insert_if_missing(&key, creator);
            self.map.get(&key).unwrap()
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key)
        }

        pub fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> CacheInsert {
            if !self.map.contains_key(&key) {
                self.map.insert(key.clone(), creator(&key));
                CacheInsert::AlreadyPresent
            } else {
                CacheInsert::Inserted
            }
        }
    }
}

mod cache_metrics {
    use std::hash::Hash;

    use crate::cache::{Cache, CacheInsert::*};

    pub struct CacheMetrics<K: Hash + Eq + Clone, V> {
        cache: Cache<K, V>,
        hits: u64,
        misses: u64,
    }

    impl<K: Hash + Eq + Clone, V> CacheMetrics<K, V> {
        pub fn new() -> CacheMetrics<K, V> {
            CacheMetrics {
                cache: Cache::new(),
                hits: 0,
                misses: 0,
            }
        }

        pub fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            match self.cache.insert_if_missing(&key, creator) {
                AlreadyPresent => self.hits += 1,
                Inserted => self.misses += 1,
            }
            self.cache.get(&key).unwrap()
        }

        pub fn hits(&self) -> u64 {
            self.hits
        }

        pub fn misses(&self) -> u64 {
            self.misses
        }
    }
}

fn main() {
    let mut cache = cache_metrics::CacheMetrics::new();
    let key = 5;
    cache.get_or_insert(key, make_something);
    cache.get_or_insert(key, make_something);
    cache.get_or_insert(key, make_something);
    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.hits(), 2);
    println!("hits: {} misses: {}", cache.hits(), cache.misses());
}

fn make_something(key: &u64) -> u64 {
    key + 1
}
