use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

struct CacheValue<T> {
    val: T,
    last_used: Instant,
}

fn add_to_cache<K: Hash + Eq + Clone + Debug, V>(
    hash: &mut HashMap<K, CacheValue<V>>,
    max_size: usize,
    key: K,
    val: V,
) -> Option<V> {
    let v = CacheValue {
        val,
        last_used: Instant::now(),
    };
    let old_val = hash.insert(key, v).map(|cv| cv.val);
    if hash.len() > max_size {
        resize_map(hash, max_size);
    }
    old_val
}

fn resize_map<K: Hash + Eq + Clone + Debug, V>(map: &mut HashMap<K, CacheValue<V>>, max_size: usize) {
    let map_len = map.len();
    let to_remove = {
        let mut items = map.iter().collect::<Vec<_>>();
        items.sort_unstable_by_key(|item| item.1.last_used);
        items
            .into_iter()
            .take(map_len - max_size)
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<K>>()
    };
    to_remove.iter().for_each(|k| {
        println!("removing: {:?}", k);
         map.remove(k);
    });
}

fn main() {
    const MAX_SIZE: usize = 4;
    let mut map = HashMap::new();

    add_to_cache(&mut map, MAX_SIZE, "one", 1);
    add_to_cache(&mut map, MAX_SIZE, "two", 2);
    add_to_cache(&mut map, MAX_SIZE, "three", 3);
    add_to_cache(&mut map, MAX_SIZE, "four", 4);
    add_to_cache(&mut map, MAX_SIZE, "five", 5);
}


fn run() {
    const MAX_SIZE: usize = 4;
    let mut map = HashMap::new();

    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
}
