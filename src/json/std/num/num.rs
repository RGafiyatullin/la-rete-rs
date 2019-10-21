use json_utils::json::JsValue;

use crate::core::fact::TypedFact;
use crate::json::JsPath;
use crate::prelude::*;

use super::IntoJsNumber;
use super::PredicateGT;
use super::PredicateGTE;
use super::PredicateLT;
use super::PredicateLTE;

pub fn eq<S: AsRef<str>, II: IntoIterator<Item = S>, N: IntoJsNumber>(
    path: II,
    value: N,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    let js_number = value.into_js_number();
    let js_value = JsValue::Number(js_number);
    TypedFact::property_is_eq(property, js_value)
}

pub fn gt<S: AsRef<str>, II: IntoIterator<Item = S>, N: IntoJsNumber>(
    path: II,
    value: N,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, PredicateGT::new(value))
}

pub fn lt<S: AsRef<str>, II: IntoIterator<Item = S>, N: IntoJsNumber>(
    path: II,
    value: N,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, PredicateLT::new(value))
}

pub fn gte<S: AsRef<str>, II: IntoIterator<Item = S>, N: IntoJsNumber>(
    path: II,
    value: N,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, PredicateGTE::new(value))
}

pub fn lte<S: AsRef<str>, II: IntoIterator<Item = S>, N: IntoJsNumber>(
    path: II,
    value: N,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, PredicateLTE::new(value))
}
