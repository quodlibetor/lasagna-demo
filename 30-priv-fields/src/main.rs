#![feature(rust_2018_preview, nll)]

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub struct Cache<K: Hash + Eq, V> {
        map: HashMap<K, V>,
        hits: usize,
        misses: usize,
    }

    impl<K: Hash + Eq + Clone, V> Cache<K, V> {
        pub fn new() -> Cache<K, V> {
            Cache {
                map: HashMap::new(),
                hits: 0,
                misses: 0,
            }
        }
        pub fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            self.ensure_exists(&key, creator);
            self.map.get(&key).unwrap()
        }

        fn ensure_exists(&mut self, key: &K, creator: impl FnOnce(&K) -> V) {
            if !self.map.contains_key(&key) {
                self.misses += 1;
                self.map.insert(key.clone(), creator(&key));
            } else {
                self.hits += 1;
            }
        }

        pub fn hits(&self) -> usize {
            self.hits
        }

        pub fn misses(&self) -> usize {
            self.misses
        }
    }
}

fn main() {
    let mut cache = cache::Cache::new();
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
