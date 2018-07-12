pub enum CacheInsert {
    AlreadyPresent,
    Inserted,
}

pub trait Cache<K, V>
where
    K: Clone,
{
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, val: V);
    fn insert_if_missing(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> CacheInsert;

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(&key).unwrap()
    }
}
