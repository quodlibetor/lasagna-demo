use crate::{cache::CacheInsert, Cache};

pub struct CascadeCache<K, V>
where
    K: Clone,
{
    first: Box<dyn Cache<K, V>>,
    rest: Vec<Box<dyn Cache<K, V>>>,
}

impl<K, V> CascadeCache<K, V>
where
    K: Clone,
{
    pub fn with(first: Box<dyn Cache<K, V>>) -> CascadeCache<K, V> {
        CascadeCache {
            first,
            rest: Vec::new(),
        }
    }

    pub fn and(mut self, cache: Box<dyn Cache<K, V>>) -> CascadeCache<K, V> {
        self.rest.push(cache);
        self
    }
}

impl<K, V> Cache<K, V> for CascadeCache<K, V>
where
    K: Clone,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<&V> {
        if let Some(val) = self.first.get(key) {
            return Some(val);
        }
        for cache in self.rest.iter() {
            if let Some(val) = cache.get(key) {
                return Some(val);
            }
        }
        None
    }

    fn insert(&mut self, key: K, val: V) {
        for cache in self.rest.iter_mut().rev() {
            cache.insert(key.clone(), val.clone());
        }
        self.first.insert(key, val);
    }

    fn insert_if_missing(&mut self, key: &K, mut creator: Box<dyn FnMut(&K) -> V>) -> CacheInsert {
        if self.first.get(key).is_some() {
            return CacheInsert::AlreadyPresent;
        }
        for cache in self.rest.iter() {
            if cache.get(key).is_some() {
                return CacheInsert::AlreadyPresent;
            }
        }

        let val = creator(key);
        self.insert(key.clone(), val);

        CacheInsert::Inserted
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(&key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add() {
        let _ = CascadeCache::<u64, u64>::with(Box::new(HashCache::new()))
            .and(Box::new(HashCache::new()));
    }
}
