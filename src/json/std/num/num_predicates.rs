use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;

use json_utils::json::JsNumber;
use json_utils::json::JsValue;

use super::IntoJsNumber;
use crate::core::fact::Predicate;

pub struct PredicateGT {
    number: JsNumber,
}
impl PredicateGT {
    pub fn new<N: IntoJsNumber>(number: N) -> Self {
        let number = number.into_js_number();
        Self { number }
    }
}
impl Predicate<JsValue> for PredicateGT {
    fn apply(&self, value: &JsValue) -> bool {
        js_value_cmp(&self.number, value) == Some(Ordering::Greater)
    }
}
pub struct PredicateLT {
    number: JsNumber,
}
impl PredicateLT {
    pub fn new<N: IntoJsNumber>(number: N) -> Self {
        let number = number.into_js_number();
        Self { number }
    }
}

impl Predicate<JsValue> for PredicateLT {
    fn apply(&self, value: &JsValue) -> bool {
        js_value_cmp(&self.number, value) == Some(Ordering::Less)
    }
}

pub struct PredicateGTE {
    number: JsNumber,
}
impl PredicateGTE {
    pub fn new<N: IntoJsNumber>(number: N) -> Self {
        let number = number.into_js_number();
        Self { number }
    }
}

impl Predicate<JsValue> for PredicateGTE {
    fn apply(&self, value: &JsValue) -> bool {
        if let Some(ord) = js_value_cmp(&self.number, value) {
            ord == Ordering::Equal || ord == Ordering::Greater
        } else {
            false
        }
    }
}
pub struct PredicateLTE {
    number: JsNumber,
}
impl PredicateLTE {
    pub fn new<N: IntoJsNumber>(number: N) -> Self {
        let number = number.into_js_number();
        Self { number }
    }
}

impl Predicate<JsValue> for PredicateLTE {
    fn apply(&self, value: &JsValue) -> bool {
        if let Some(ord) = js_value_cmp(&self.number, value) {
            ord == Ordering::Equal || ord == Ordering::Less
        } else {
            false
        }
    }
}

fn js_value_cmp(left: &JsNumber, right: &JsValue) -> Option<Ordering> {
    match *right {
        JsValue::Number(ref right) => num_cmp(left, right),
        _ => None,
    }
}

fn num_cmp(left: &JsNumber, right: &JsNumber) -> Option<Ordering> {
    if let (Some(left), Some(right)) = (left.as_i64(), right.as_i64()) {
        Some(left.cmp(&right))
    } else if let (Some(left), Some(right)) = (left.as_u64(), right.as_u64()) {
        Some(left.cmp(&right))
    } else if let (Some(left), Some(right)) = (left.as_f64(), right.as_f64()) {
        left.partial_cmp(&right)
    } else {
        unimplemented!()
    }
}
