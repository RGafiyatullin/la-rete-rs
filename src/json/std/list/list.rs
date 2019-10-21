use json_utils::json::JsValue;

use crate::core::fact::TypedFact;
use crate::json::JsPath;
use crate::prelude::*;

use super::LenEq;
use super::LenGT;
use super::LenGTE;
use super::LenLT;
use super::LenLTE;

pub fn len_eq<P: AsRef<str>, II: IntoIterator<Item = P>>(
    path: II,
    len: usize,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, LenEq::new(len))
}

pub fn len_lt<P: AsRef<str>, II: IntoIterator<Item = P>>(
    path: II,
    len: usize,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, LenLT::new(len))
}

pub fn len_gt<P: AsRef<str>, II: IntoIterator<Item = P>>(
    path: II,
    len: usize,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, LenGT::new(len))
}

pub fn len_lte<P: AsRef<str>, II: IntoIterator<Item = P>>(
    path: II,
    len: usize,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, LenLTE::new(len))
}

pub fn len_gte<P: AsRef<str>, II: IntoIterator<Item = P>>(
    path: II,
    len: usize,
) -> impl Fact<JsValue> {
    let property = JsPath::from_path(path);
    TypedFact::predicate(property, LenGTE::new(len))
}
