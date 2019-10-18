use json_utils::json::JsValue;

use crate::core::fact::TypedFact;
use crate::json::JsPath;
use crate::prelude::*;

pub fn eq<S: AsRef<str>, II: IntoIterator<Item = S>>(
    path: II,
    value: JsValue,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::property_is_eq(property, value)
}
