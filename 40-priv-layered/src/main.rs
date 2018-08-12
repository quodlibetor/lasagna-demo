#![feature(rust_2018_preview, nll)]

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub(crate) enum Insert {
        AlreadyPresent,
        Inserted,
    }

    pub(crate) struct Cache<K: Hash + Eq, V> {
        map: HashMap<K, V>,
    }

    impl<K: Hash + Eq + Clone, V> Cache<K, V> {
        pub(crate) fn new() -> Cache<K, V> {
            Cache {
                map: HashMap::new(),
            }
        }

        pub(crate) fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            self.insert_if_missing(&key, creator);
            self.map.get(&key).unwrap()
        }

        pub(crate) fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key)
        }

        pub(crate) fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> Insert {
            if !self.map.contains_key(&key) {
                self.map.insert(key.clone(), creator(&key));
                Insert::AlreadyPresent
            } else {
                Insert::Inserted
            }
        }
    }
}

mod cache_metrics {
    use std::hash::Hash;

    use cache::{Cache, Insert};

    pub(crate) struct CacheMetrics<K: Hash + Eq + Clone, V> {
        cache: Cache<K, V>,
        hits: u64,
        misses: u64,
    }

    impl<K: Hash + Eq + Clone, V> CacheMetrics<K, V> {
        pub(crate) fn new() -> CacheMetrics<K, V> {
            CacheMetrics {
                cache: Cache::new(),
                hits: 0,
                misses: 0,
            }
        }

        pub(crate) fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            match self.cache.insert_if_missing(&key, creator) {
                Insert::AlreadyPresent => self.hits += 1,
                Insert::Inserted => self.misses += 1,
            }
            self.cache.get(&key).unwrap()
        }

        pub(crate) fn hits(&self) -> u64 {
            self.hits
        }

        pub(crate) fn misses(&self) -> u64 {
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
