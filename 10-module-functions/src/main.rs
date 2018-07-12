#![feature(rust_2018_preview)]

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    type CacheMap<K, V> = HashMap<K, V>;

    crate fn new<K, V>() -> CacheMap<K, V>
    where
        K: Hash + Eq + Clone,
    {
        CacheMap::new()
    }

    crate fn get_or_insert<K, V>(
        cache: &mut CacheMap<K, V>,
        key: K,
        creator: impl FnOnce(&K) -> V,
    ) -> &V
    where
        K: Hash + Eq + Clone,
    {
        ensure_exists(cache, &key, creator);
        cache.get(&key).unwrap()
    }

    fn ensure_exists<K, V>(cache: &mut CacheMap<K, V>, key: &K, creator: impl FnOnce(&K) -> V)
    where
        K: Hash + Eq + Clone,
    {
        if !cache.contains_key(&key) {
            cache.insert(key.clone(), creator(&key));
        }
    }
}

#[derive(Debug, PartialEq)]
struct HardToCreate(usize);

fn main() {
    let mut cachemap = cache::new();

    let val = cache::get_or_insert(&mut cachemap, "one", |_| HardToCreate(1));
    assert_eq!(val, &HardToCreate(1));
}
