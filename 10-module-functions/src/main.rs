#![feature(rust_2018_preview)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]

use std::collections::HashMap;

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    type CacheMap<K, V> = HashMap<K, V>;

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

fn main() {
    let mut cachemap = HashMap::new();

    let val = cache::get_or_insert(&mut cachemap, "one", |k| k.len());
    assert_eq!(val, &3);
}
