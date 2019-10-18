use json_utils::json::JsValue;

use crate::core::ruleset::*;

use super::JsonFilter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRule {
    #[serde(rename = "if")]
    filter: JsonFilter,

    #[serde(rename = "then")]
    value: JsValue,
}

impl JsonRule {
    pub fn into_rule(self) -> Rule<JsValue, JsValue> {
        let filter = self.filter.into_filter();
        Rule::new(self.value).with_filter(filter)
    }
}
