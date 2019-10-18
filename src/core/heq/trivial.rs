use super::HEq;

pub struct HEqTrivial;

impl HEqTrivial {
    pub fn calc_hash<T: std::hash::Hash>(value: &T) -> u64 {
        use std::hash::Hasher;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    pub fn are_eq<T: Eq>(left: &T, right: &T) -> bool {
        left == right
    }
}

macro_rules! impl_heq {
    ($type: ty) => {
        impl HEq<$type> for HEqTrivial {
            fn are_eq(&self, left: &$type, right: &$type) -> bool {
                Self::are_eq(left, right)
            }

            fn calc_hash(&self, value: &$type) -> u64 {
                Self::calc_hash(value)
            }
        }
    };
}

impl_heq!(u8);
impl_heq!(i8);
impl_heq!(u16);
impl_heq!(i16);
impl_heq!(u32);
impl_heq!(i32);
impl_heq!(u64);
impl_heq!(i64);
impl_heq!(u128);
impl_heq!(i128);

impl_heq!(String);
impl_heq!(&[u8]);
