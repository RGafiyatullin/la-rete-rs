use std::sync::Arc;

pub struct KV<K, V> {
    key: Arc<K>,
    value: V,
}

impl<K, V> KV<K, V> {
    pub fn new(key: Arc<K>, value: V) -> Self {
        Self { key, value }
    }

    field_get_ref!(key, key, K);
    field_get_ref!(value, value, V);
    field_get_ref_mut!(value, value_mut, V);

    pub fn into_tuple(self) -> (Arc<K>, V) {
        (self.key, self.value)
    }
}
