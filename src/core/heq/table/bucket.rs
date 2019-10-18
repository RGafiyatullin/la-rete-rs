use std::sync::Arc;

use super::HEq;
use super::KV;

pub struct Bucket<K, V> {
    elements: Vec<KV<K, V>>,
}

impl<K, V> Bucket<K, V> {
    pub fn new() -> Self {
        let elements = Vec::with_capacity(0);
        Self { elements }
    }

    pub fn find<'a>(&'a self, heq: &dyn HEq<K>, key: &K) -> Option<&'a V> {
        self.elements
            .iter()
            .find(move |kv| heq.are_eq(key, kv.key()))
            .map(|kv| kv.value())
    }

    pub fn find_mut<'a>(&'a mut self, heq: &dyn HEq<K>, key: &K) -> Option<&'a mut V> {
        self.elements
            .iter_mut()
            .find(move |kv| heq.are_eq(key, kv.key()))
            .map(|kv| kv.value_mut())
    }

    pub fn insert(&mut self, heq: &dyn HEq<K>, key: Arc<K>, value: V) -> Result<(), (Arc<K>, V)> {
        if let Some(_) = self.find(heq, &key) {
            Err((key, value))
        } else {
            let kv = KV::new(key, value);
            self.elements.push(kv);
            Ok(())
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = KV<K, V>> {
        self.elements.into_iter()
    }
}
