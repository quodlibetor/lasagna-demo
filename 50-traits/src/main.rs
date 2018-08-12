mod cache;
mod cache_metrics;
mod hash_cache;

use cache::Cache;
use cache_metrics::CacheMetrics;
use hash_cache::HashCache;

fn main() {
    let mut cache = CacheMetrics::with(HashCache::new());
    let key = 5;
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    do_something(&mut cache, key);

    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.hits(), 2);
}

fn do_something(cache: &mut impl Cache<u64, String>, key: u64) {
    cache.get_or_insert(&key, do_something_expensive);
}

fn do_something_expensive(key: &u64) -> String {
    key.to_string()
}
