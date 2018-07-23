#![feature(rust_2018_preview)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]

use std::collections::HashMap;

mod cache {
    use std::collections::HashMap;
    use std::hash::Hash;

    type CacheMap<K, V> = HashMap<K, V>;

    fn ensure_exists<K, V>(cache: &mut HashMap<K, V>, key: &K, creator: impl FnOnce(&K) -> V)
    where
        K: Hash + Eq + Clone,
    {...}
}

fn get_or_insert<K, V>(
    cache: &mut HashMap<K, V>,
    key: K,
    creator: impl FnOnce(&K) -> V,
) -> &V
where K: Hash + Eq + Clone,
{...}


fn do_something(cache: &mut HashMap<u32, String>, key: u32) {
    if !cache.contains_key(key) {
        let value = do_something_expensive(key);
        cache.insert(key, value);
    }
    let val = cache.get(key).unwrap();
    // real work with val
}
fn main() {
    let mut cache = HashMap::new();
    do_something(&mut cache, 1_000);
    do_something(&mut cache, 2);
    do_something(&mut cache, 1_000);
}
