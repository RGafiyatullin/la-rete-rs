pub trait Predicate<V> {
    fn apply(&self, value: &V) -> bool;
}
