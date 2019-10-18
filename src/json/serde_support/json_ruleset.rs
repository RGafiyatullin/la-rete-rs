use json_utils::json::JsValue;

use crate::core::ruleset::*;

use super::JsonRule;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRuleset(Vec<JsonRule>);

impl JsonRuleset {
    pub fn into_ruleset(self) -> Ruleset<JsValue, JsValue> {
        let rules = self.0.into_iter().map(|jr| jr.into_rule()).collect();
        Ruleset::new().with_rules(rules)
    }
}
