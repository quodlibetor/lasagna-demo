#![feature(rust_2018_preview, nll)]

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub(crate) struct Cache<K: Hash + Eq, V> {
        pub(crate) map: HashMap<K, V>,
    }

    impl<K: Hash + Eq + Clone, V> Cache<K, V> {
        pub(crate) fn new() -> Cache<K, V> {
            Cache {
                map: HashMap::new(),
            }
        }
        pub(crate) fn get_or_insert(&mut self, key: K, creator: impl FnOnce(&K) -> V) -> &V {
            self.ensure_exists(&key, creator);
            self.map.get(&key).unwrap()
        }

        fn ensure_exists(&mut self, key: &K, creator: impl FnOnce(&K) -> V) {
            if !self.map.contains_key(&key) {
                self.map.insert(key.clone(), creator(&key));
            }
        }
    }
}

fn main() {
    let mut cache = cache::Cache::new();
    let key = 5;
    let val = cache.get_or_insert(key, make_something).clone();
    assert_eq!(*cache.map.get(&key).unwrap(), val);
}

fn make_something(key: &u64) -> u64 {
    key + 1
}
