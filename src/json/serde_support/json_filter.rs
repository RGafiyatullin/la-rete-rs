use json_utils::json::JsValue;

use crate::core::fact::TypedFact;
use crate::core::ruleset::*;
use crate::json::JsPath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JsonFilter {
    #[serde(rename = "or")]
    Or(Vec<JsonFilter>),

    #[serde(rename = "and")]
    And(Vec<JsonFilter>),

    #[serde(rename = "eq")]
    Eq { path: String, value: JsValue },

    #[serde(rename = "neq")]
    Neq { path: String, value: JsValue },
}

impl JsonFilter {
    pub fn into_filter(self) -> Filter<JsValue> {
        match self {
            Self::Eq { path, value } => {
                let path = parse_path(&path);
                let property = JsPath::from_path(path);
                let fact = TypedFact::property_is_eq(property, value);
                Filter::Fact(Box::new(fact))
            }
            Self::Neq { path, value } => {
                let path = parse_path(&path);
                let property = JsPath::from_path(path);
                let fact = TypedFact::proeprty_is_neq(property, value);
                Filter::Fact(Box::new(fact))
            }
            Self::And(filters) => {
                Filter::And(filters.into_iter().map(|jf| jf.into_filter()).collect())
            }
            Self::Or(filters) => {
                Filter::Or(filters.into_iter().map(|jf| jf.into_filter()).collect())
            }
        }
    }
}

fn parse_path(s: &str) -> Vec<String> {
    s.split("/").map(|s| s.to_owned()).collect()
}
