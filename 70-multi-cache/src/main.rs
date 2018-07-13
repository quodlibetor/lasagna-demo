#![feature(rust_2018_preview, nll)]

use redis::Command;

use crate::cache::Cache;
use crate::cache_metrics::CacheMetrics;
use crate::hash_cache::HashCache;
use crate::cascade_cache::CascadeCache;

mod cache;
mod cache_metrics;
mod cascade_cache;
mod hash_cache;

fn main() {
    let mut cache = CacheMetrics::with(CascadeCache::with(Box::new(HashCache::new())));
    let key = 5;
    cache.get_or_insert(&key, Box::new(make_something));
    cache.get_or_insert(&key, Box::new(make_something));
    cache.get_or_insert(&key, Box::new(make_something));
    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.hits(), 2);
    println!("hits: {} misses: {}", cache.hits(), cache.misses());
}

fn make_something(key: &u64) -> u64 {
    key + 1
}
