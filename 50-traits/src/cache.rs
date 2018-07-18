pub enum CacheInsert {
    AlreadyPresent,
    Inserted,
}

pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> CacheInsert
    where
        K: Clone;

    fn get_or_insert(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> &V
    where
        K: Clone,
    {
        self.insert_if_missing(key, creator);
        self.get(&key).unwrap()
    }
}
