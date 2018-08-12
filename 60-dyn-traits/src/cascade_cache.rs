use {cache::Insert, Cache};

type AnyCache<K, V> = Box<dyn Cache<K, V>>;

pub(crate) struct CascadeCache<K, V> {
    first: AnyCache<K, V>,
    rest: Vec<AnyCache<K, V>>,
}

impl<K, V> CascadeCache<K, V> {
    pub(crate) fn with(first: AnyCache<K, V>) -> CascadeCache<K, V> {
        CascadeCache {
            first,
            rest: Vec::new(),
        }
    }

    pub(crate) fn and(mut self, cache: AnyCache<K, V>) -> CascadeCache<K, V> {
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

    fn insert_if_missing(&mut self, key: &K, mut creator: Box<dyn FnMut(&K) -> V>) -> Insert {
        if self.first.get(key).is_some() {
            return Insert::AlreadyPresent;
        }
        for cache in self.rest.iter() {
            if cache.get(key).is_some() {
                return Insert::AlreadyPresent;
            }
        }

        let val = creator(key);
        self.insert(key.clone(), val);

        Insert::Inserted
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(&key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use HashCache;

    #[test]
    fn can_add() {
        let _ = CascadeCache::<u64, u64>::with(Box::new(HashCache::new()))
            .and(Box::new(HashCache::new()));
    }
}
