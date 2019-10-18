use std::sync::Arc;

use super::HEq;
use super::Table;

struct IntHeq;
impl HEq<i32> for IntHeq {
    fn are_eq(&self, left: &i32, right: &i32) -> bool {
        *left == *right
    }
    fn calc_hash(&self, value: &i32) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[test]
fn test_insert_and_lookup() {
    let data: Vec<(Arc<i32>, &'static str)> = vec![
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
    ]
    .into_iter()
    .map(|(k, v)| (Arc::new(k), v))
    .collect();

    let heq = Arc::new(IntHeq);

    let mut table = Table::<i32, &'static str>::with_capacity(heq, 3);

    for (key, _value) in data.iter().cloned() {
        assert!(table.lookup(&key).is_none());
    }
    for (key, value) in data.iter().cloned() {
        assert!(table.insert(key, value).is_ok());
    }
    for (key, value) in data.iter().cloned() {
        assert!(table.lookup(&key) == Some(&value));
    }
    for (key, value) in data.iter().cloned() {
        assert!(table.insert(key, value).is_err());
    }
}
