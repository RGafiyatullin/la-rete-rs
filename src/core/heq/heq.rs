use std::sync::Arc;

pub trait HEq<T> {
    fn are_eq(&self, left: &T, right: &T) -> bool;
    fn calc_hash(&self, value: &T) -> u64;
}

pub type ArcDynHEq<T> = Arc<dyn HEq<T>>;
