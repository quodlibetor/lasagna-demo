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
    K: Hash + Eq + Clone,
{
    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V
    where
        K: Clone,
    {
        self.insert_if_missing(&key, creator);
        self.map.get(&key).unwrap()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn insert(&mut self, key: K, val: V) {
        self.map.insert(key.clone(), val);
    }

    fn insert_if_missing(&mut self, key: &K, mut creator: Box<dyn FnMut(&K) -> V>) -> CacheInsert
    where
        K: Clone,
    {
        if !self.map.contains_key(&key) {
            let created = creator(&key);
            self.insert(key.clone(), created);
            CacheInsert::AlreadyPresent
        } else {
            CacheInsert::Inserted
        }
    }
}
