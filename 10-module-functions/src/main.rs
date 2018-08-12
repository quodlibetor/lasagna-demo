#![feature(rust_2018_preview)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]

use std::collections::HashMap;

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    type CacheMap<K, V> = HashMap<K, V>;

    pub(crate) fn get_or_insert<K, V>(
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

fn do_something(cache: &mut HashMap<u32, String>, key: u32) {
    let val = cache::get_or_insert(cache, key, do_something_expensive);
    // real work with val
}

fn main() {
    let mut cache = HashMap::new();
    do_something(&mut cache, 1_000);
    do_something(&mut cache, 2);
    do_something(&mut cache, 1_000);
}

fn do_something_expensive(key: &u32) -> String {
    key.to_string()
}
