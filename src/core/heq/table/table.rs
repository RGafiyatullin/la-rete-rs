use std::sync::Arc;

use super::ArcDynHEq;
use super::Bucket;

const TAB_CAPACITY: usize = 256;

pub struct Table<K, V> {
    heq: ArcDynHEq<K>,
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> Table<K, V> {
    pub fn new(heq: ArcDynHEq<K>) -> Self {
        Self::with_capacity(heq, TAB_CAPACITY)
    }

    pub fn with_capacity(heq: ArcDynHEq<K>, capacity: usize) -> Self {
        let buckets = (1..capacity).map(|_| Bucket::new()).collect();

        Self { heq, buckets }
    }

    pub fn insert(&mut self, key: Arc<K>, value: V) -> Result<(), (Arc<K>, V)> {
        let buckets_len = self.buckets.len();
        let key_hash = self.heq.calc_hash(&key);
        let bucket = &mut self.buckets[(key_hash % buckets_len as u64) as usize];
        bucket.insert(self.heq.as_ref(), key, value)
    }

    pub fn lookup<'a>(&'a self, key: &K) -> Option<&'a V> {
        let buckets_len = self.buckets.len();
        let key_hash = self.heq.calc_hash(key);
        let bucket = &self.buckets[(key_hash % buckets_len as u64) as usize];
        bucket.find(self.heq.as_ref(), key)
    }

    pub fn lookup_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        let buckets_len = self.buckets.len();
        let key_hash = self.heq.calc_hash(key);
        let bucket = &mut self.buckets[(key_hash % buckets_len as u64) as usize];
        bucket.find_mut(self.heq.as_ref(), key)
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Arc<K>, V)> {
        self.buckets
            .into_iter()
            .map(|bucket| bucket.into_iter())
            .flatten()
            .map(|kv| kv.into_tuple())
    }
}
