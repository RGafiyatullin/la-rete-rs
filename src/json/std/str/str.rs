use json_utils::json::JsValue;

use crate::core::fact::TypedFact;
use crate::json::JsPath;
use crate::prelude::*;

pub fn eq<P: AsRef<str>, II: IntoIterator<Item = P>, S: AsRef<str>>(
    path: II,
    value: S,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    let string = value.as_ref().to_owned();
    let js_value = JsValue::String(string);
    TypedFact::property_is_eq(property, js_value)
}
