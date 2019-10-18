use std::hash::Hasher;

pub use json_utils::json::JsValue;

const HASH_PREFIX_NULL: u8 = 0;
const HASH_PREFIX_BOOL: u8 = 1;
const HASH_PREFIX_NUMBER: u8 = 2;
const HASH_PREFIX_STRING: u8 = 3;
const HASH_PREFIX_ARRAY: u8 = 4;
const HASH_PREFIX_OBJECT: u8 = 5;

pub fn js_value_hash<H: Hasher>(value: &JsValue, hasher: &mut H) {
    match value {
        JsValue::Null => hasher.write_u8(HASH_PREFIX_NULL),
        JsValue::Bool(false) => hasher.write(&[HASH_PREFIX_BOOL, 0]),
        JsValue::Bool(true) => hasher.write(&[HASH_PREFIX_BOOL, 1]),
        JsValue::String(string) => {
            hasher.write_u8(HASH_PREFIX_STRING);
            hasher.write(string.as_bytes());
        }
        JsValue::Number(number) => {
            hasher.write_u8(HASH_PREFIX_NUMBER);
            if let Some(u) = number.as_u64() {
                hasher.write_u64(u)
            } else if let Some(i) = number.as_i64() {
                hasher.write_i64(i)
            } else if let Some(f) = number.as_f64() {
                hasher.write_u64(f.to_bits())
            } else {
                unreachable!()
            }
        }
        JsValue::Array(ref values) => {
            hasher.write_u8(HASH_PREFIX_ARRAY);
            for v in values {
                js_value_hash(v, hasher)
            }
        }
        JsValue::Object(ref fields) => {
            hasher.write_u8(HASH_PREFIX_OBJECT);
            let mut keys = fields.keys().collect::<Vec<_>>();
            keys.sort_unstable();
            for key in keys {
                hasher.write(key.as_bytes());
                if let Some(value) = fields.get(key) {
                    js_value_hash(value, hasher)
                } else {
                    unreachable!("The key from the keys-iterator does not exist in the map")
                }
            }
        }
    }
}
