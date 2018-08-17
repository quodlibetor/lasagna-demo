pub(crate) enum Insert {
    AlreadyPresent,
    Inserted,
}

pub(crate) trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, val: V);
    fn insert_if_missing(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Insert;

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(&key).unwrap()
    }
}
