use json_utils::json::JsValue;
use serde_json::Error as SerdeError;

use crate::core::ruleset::Ruleset;

mod json_filter;
pub use json_filter::JsonFilter;

mod json_rule;
pub use json_rule::JsonRule;

mod json_ruleset;
pub use json_ruleset::JsonRuleset;

pub fn parse_ruleset(input: &[u8]) -> Result<Ruleset<JsValue, JsValue>, SerdeError> {
    serde_json::from_slice::<JsonRuleset>(input).map(|jrs| jrs.into_ruleset())
}
