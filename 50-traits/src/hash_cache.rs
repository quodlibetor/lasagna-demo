use std::collections::HashMap;
use std::hash::Hash;

use cache::Cache;
use cache::Insert;

pub(crate) struct HashCache<K: Hash + Eq, V> {
    map: HashMap<K, V>,
}

impl<K: Hash + Eq, V> HashCache<K, V> {
    pub(crate) fn new() -> HashCache<K, V> {
        HashCache {
            map: HashMap::new(),
        }
    }
}

impl<K, V> Cache<K, V> for HashCache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn get_or_insert(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> &V {
        self.insert_if_missing(&key, creator);
        self.map.get(&key).unwrap()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> Insert {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), creator(&key));
            Insert::Inserted
        } else {
            Insert::AlreadyPresent
        }
    }
}
