use std::cmp::Ord;
use std::cmp::Ordering;

use json_utils::json::JsValue;

use crate::core::fact::Predicate;

#[derive(new)]
pub struct LenEq {
    len: usize,
}

#[derive(new)]
pub struct LenLT {
    len: usize,
}

#[derive(new)]
pub struct LenGT {
    len: usize,
}

#[derive(new)]
pub struct LenLTE {
    len: usize,
}

#[derive(new)]
pub struct LenGTE {
    len: usize,
}

impl Predicate<JsValue> for LenEq {
    fn apply(&self, value: &JsValue) -> bool {
        let cmp = list_len_cmp(value, self.len);
        cmp == Some(Ordering::Equal)
    }
}
impl Predicate<JsValue> for LenLT {
    fn apply(&self, value: &JsValue) -> bool {
        let cmp = list_len_cmp(value, self.len);
        cmp == Some(Ordering::Less)
    }
}
impl Predicate<JsValue> for LenGT {
    fn apply(&self, value: &JsValue) -> bool {
        let cmp = list_len_cmp(value, self.len);
        cmp == Some(Ordering::Greater)
    }
}
impl Predicate<JsValue> for LenLTE {
    fn apply(&self, value: &JsValue) -> bool {
        let cmp = list_len_cmp(value, self.len);
        cmp == Some(Ordering::Equal) || cmp == Some(Ordering::Less)
    }
}
impl Predicate<JsValue> for LenGTE {
    fn apply(&self, value: &JsValue) -> bool {
        let cmp = list_len_cmp(value, self.len);
        cmp == Some(Ordering::Equal) || cmp == Some(Ordering::Greater)
    }
}

pub fn list_len_cmp(value: &JsValue, expected_len: usize) -> Option<Ordering> {
    let as_list = match *value {
        JsValue::Array(ref inner) => Some(inner),
        _ => None,
    };
    as_list.map(|list| list.len().cmp(&expected_len))
}
