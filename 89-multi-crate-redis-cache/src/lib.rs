#![feature(rust_2018_preview, nll)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]

use redis::{Commands, RedisError, RedisResult};

use remote_cache::{RemoteCache, RemoteInsert};

pub struct RedisCache {
    conn: redis::Connection,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> redis::RedisResult<RedisCache> {
        Ok(RedisCache {
            conn: redis::Client::open(redis_url)?.get_connection()?,
        })
    }
}

// TODO: try to resolve the conflicting impl by moving the struct to its own create
impl<K, V> RemoteCache<K, V> for RedisCache
where
    K: redis::ToRedisArgs + Clone,
    V: redis::ToRedisArgs + redis::FromRedisValue,
{
    type Err = RedisError;
    fn get(&self, key: &K) -> RedisResult<Option<V>> {
        self.conn.get(*key).unwrap()
    }

    fn insert(&mut self, key: K, val: V) {
        self.conn.set(key, val).unwrap()
    }
    fn insert_if_missing(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> RedisResult<RemoteInsert<V>> {
        match self.get(key)? {
            Some(val) => Ok(RemoteInsert::AlreadyPresent(val)),
            None => {
                let val = creator(key);
                self.insert(key.clone(), val);
                RemoteInsert::Inserted(val)
            }
        }
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(key).unwrap()
    }
}
