use json_utils::json::JsValue;

use crate::core::heq::HEq;

mod js_value_eq;
use js_value_eq::js_value_eq;

mod js_value_hash;
use js_value_hash::js_value_hash;

pub struct JsValueHEq;

impl HEq<JsValue> for JsValueHEq {
    fn are_eq(&self, left: &JsValue, right: &JsValue) -> bool {
        js_value_eq(left, right)
    }

    fn calc_hash(&self, value: &JsValue) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        js_value_hash(value, &mut hasher);
        hasher.finish()
    }
}
