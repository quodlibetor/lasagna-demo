use std::collections::HashMap;
use std::hash::Hash;

use crate::cache::Cache;
use crate::cache::CacheInsert;

pub struct HashCache<K: Hash + Eq, V> {
    map: HashMap<K, V>,
}

impl<K: Hash + Eq, V> HashCache<K, V> {
    pub fn new() -> HashCache<K, V> {
        HashCache {
            map: HashMap::new(),
        }
    }
}

impl<K, V> Cache<K, V> for HashCache<K, V>
where
    K: Hash + Eq,
{
    fn get_or_insert(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> &V
    where
        K: Clone,
    {
        self.insert_if_missing(&key, creator);
        self.map.get(&key).unwrap()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> CacheInsert
    where
        K: Clone,
    {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), creator(&key));
            CacheInsert::AlreadyPresent
        } else {
            CacheInsert::Inserted
        }
    }
}
