use {cache::Insert, Cache};

crate enum RemoteInsert<V> {
    Present(V),
    Inserted(V),
}

crate trait RemoteCache<K, V> {
    type Err;
    fn get(&self, key: &K) -> Result<Option<V>, Self::Err>;
    fn insert(&mut self, key: &K, val: &V) -> Result<V, Self::Err>;

    fn insert_if_missing(
        &mut self,
        key: &K,
        creator: Box<dyn FnMut(&K) -> V>,
    ) -> Result<RemoteInsert<V>, Self::Err>;

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Result<V, Self::Err>;
}

type Infallible<T> = Result<T, ()>;

impl<T, K, V> RemoteCache<K, V> for T
where
    T: Cache<K, V>,
    V: Clone,
    K: Clone,
{
    type Err = ();
    fn get(&self, key: &K) -> Infallible<Option<V>> {
        Ok(<Self as Cache<K, V>>::get(self, key).map(Clone::clone))
    }

    fn insert(&mut self, key: &K, val: &V) -> Infallible<V> {
        <Self as Cache<K, V>>::insert(self, *key, val.clone());
        Ok(self.get(key).unwrap().clone())
    }

    fn insert_if_missing(
        &mut self,
        key: &K,
        creator: Box<dyn FnMut(&K) -> V>,
    ) -> Infallible<RemoteInsert<V>> {
        use self::Insert::*;
        match <Self as Cache<K, V>>::insert_if_missing(self, key, creator) {
            AlreadyPresent => Ok(RemoteInsert::Present(self.get(key).unwrap().clone())),
            Inserted => Ok(RemoteInsert::Inserted(self.get(key).unwrap().clone())),
        }
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Infallible<V> {
        self.insert_if_missing(key, creator);
        Ok(self.get(&key).unwrap().clone())
    }
}
