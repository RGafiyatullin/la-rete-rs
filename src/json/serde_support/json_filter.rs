use json_utils::json::JsValue;

use crate::core::ruleset::*;
use crate::json::std as j;

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

    #[serde(rename = "list::len_eq")]
    ListLenEq { path: String, value: usize },
    #[serde(rename = "list::len_gt")]
    ListLenGt { path: String, value: usize },
    #[serde(rename = "list::len_gte")]
    ListLenGte { path: String, value: usize },
    #[serde(rename = "list::len_lt")]
    ListLenLt { path: String, value: usize },
    #[serde(rename = "list::len_lte")]
    ListLenLte { path: String, value: usize },

    #[serde(rename = "num::eq")]
    NumEq { path: String, value: usize },
    #[serde(rename = "num::lt")]
    NumLt { path: String, value: usize },
    #[serde(rename = "num::lte")]
    NumLte { path: String, value: usize },
    #[serde(rename = "num::gt")]
    NumGt { path: String, value: usize },
    #[serde(rename = "num::gte")]
    NumGte { path: String, value: usize },

    #[serde(rename = "str::eq")]
    StrEq { path: String, value: String },
}

impl JsonFilter {
    pub fn into_filter(self) -> Filter<JsValue> {
        match self {
            Self::Eq { path, value } => Filter::fact(j::eq(parse_path(&path), value)),
            Self::Neq { path, value } => Filter::fact(j::neq(parse_path(&path), value)),
            Self::And(filters) => {
                Filter::And(filters.into_iter().map(|jf| jf.into_filter()).collect())
            }
            Self::Or(filters) => {
                Filter::Or(filters.into_iter().map(|jf| jf.into_filter()).collect())
            }

            Self::StrEq { path, value } => Filter::fact(j::str::eq(parse_path(&path), value)),

            Self::NumEq { path, value } => Filter::fact(j::num::eq(parse_path(&path), value)),
            Self::NumLt { path, value } => Filter::fact(j::num::lt(parse_path(&path), value)),
            Self::NumLte { path, value } => Filter::fact(j::num::lte(parse_path(&path), value)),
            Self::NumGt { path, value } => Filter::fact(j::num::gt(parse_path(&path), value)),
            Self::NumGte { path, value } => Filter::fact(j::num::gte(parse_path(&path), value)),

            Self::ListLenEq { path, value } => {
                Filter::fact(j::list::len_eq(parse_path(&path), value))
            }
            Self::ListLenLt { path, value } => {
                Filter::fact(j::list::len_lt(parse_path(&path), value))
            }
            Self::ListLenLte { path, value } => {
                Filter::fact(j::list::len_lte(parse_path(&path), value))
            }
            Self::ListLenGt { path, value } => {
                Filter::fact(j::list::len_gt(parse_path(&path), value))
            }
            Self::ListLenGte { path, value } => {
                Filter::fact(j::list::len_gte(parse_path(&path), value))
            }
        }
    }
}

fn parse_path(s: &str) -> Vec<String> {
    s.split("/").map(|s| s.to_owned()).collect()
}
