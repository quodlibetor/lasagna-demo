#![feature(rust_2018_preview, nll)]

mod cache;
mod hash_cache;
mod cache_metrics;

use crate::cache::Cache;
use crate::hash_cache::HashCache;
use crate::cache_metrics::CacheMetrics;

fn main() {
    let mut cache = CacheMetrics::with(HashCache::new());
    let key = 5;
    cache.get_or_insert(&key, make_something);
    cache.get_or_insert(&key, make_something);
    cache.get_or_insert(&key, make_something);
    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.hits(), 2);
    println!("hits: {} misses: {}", cache.hits(), cache.misses());
}

fn make_something(key: &u64) -> u64 {
    key + 1
}
