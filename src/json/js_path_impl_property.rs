use std::sync::Arc;

use json_utils::json::JsValue;
use json_utils::query::Query;

use crate::core::heq::HEq;
use crate::core::property::Property;
use crate::core::property::Value;

use super::JsPath;
use super::JsValueHEq;

impl Property<JsValue, JsValue> for JsPath {
    fn id(&self) -> String {
        format!("JsPath{:?}", self.path)
    }

    fn heq(&self) -> Arc<dyn HEq<JsValue>> {
        Arc::new(JsValueHEq)
    }

    fn value<'a>(&self, input: &'a JsValue) -> Value<'a, JsValue> {
        input
            .lookup(&self.path)
            .map(Value::Ref)
            .unwrap_or(Value::Val(JsValue::Null))
    }
}
