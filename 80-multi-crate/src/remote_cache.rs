use {cache::Insert, Cache};

pub enum RemoteInsert<V> {
    Present(V),
    Inserted(V),
}

pub trait RemoteCache<K, V> {
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
pub struct ImplPls;
pub struct DoNotImpl;
trait DoImpl {}

pub trait IntoCache<K, V> {
    type C: Cache<K, V>;
    fn into_cache(self) -> Self::C;
    fn as_cache(&self) -> &Self::C;
    fn as_mut_cache(&mut self) -> &mut Self::C;
}

impl<T, K, V> RemoteCache<K, V> for T
where
    T: IntoCache<K, V> + 'static,
    V: Clone,
    K: Clone,
{
    type Err = ();
    fn get(&self, key: &K) -> Infallible<Option<V>> {
        Ok(self.as_cache().get(key).map(Clone::clone))
    }

    fn insert(&mut self, key: &K, val: &V) -> Infallible<V> {
        self.as_mut_cache().insert(key.clone(), val.clone());
        Ok(self.as_cache().get(key).unwrap().clone())
    }

    fn insert_if_missing(
        &mut self,
        key: &K,
        creator: Box<dyn FnMut(&K) -> V>,
    ) -> Infallible<RemoteInsert<V>> {
        use self::Insert::*;
        match self.as_mut_cache().insert_if_missing(key, creator) {
            AlreadyPresent => Ok(RemoteInsert::Present(self.as_cache().get(key).unwrap().clone())),
            Inserted => Ok(RemoteInsert::Inserted(self.as_cache().get(key).unwrap().clone())),
        }
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Infallible<V> {
        self.as_mut_cache().insert_if_missing(key, creator);
        Ok(self.as_mut_cache().get(&key).unwrap().clone())
    }
}
