use redis::Commands;

use {Cache, Insert};

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

impl<K, V> Cache<K, V> for RedisCache
where
    K: redis::ToRedisArgs + Clone,
    V: redis::ToRedisArgs + redis::FromRedisValue,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.conn.get(key.clone()).unwrap()
    }
    fn insert(&mut self, key: K, val: V) {
        self.conn.set(key, val).unwrap()
    }
    fn insert_if_missing(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> Insert {
        if self.get(key).is_some() {
            return Insert::AlreadyPresent;
        }
        let val = creator(key);
        self.insert(key.clone(), val);
        Insert::Inserted
    }

    fn get_or_insert(&mut self, key: &K, creator: Box<dyn FnMut(&K) -> V>) -> &V {
        self.insert_if_missing(key, creator);
        self.get(key).unwrap()
    }
}
