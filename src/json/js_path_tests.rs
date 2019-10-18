use crate::core::property::Property;
use json_utils::json::JsValue;

use super::JsPath;

#[test]
fn js_path_creation_from_various_args() {
    let _prop_a = JsPath::from_path(vec!["a"]);

    let b = "b".to_owned();
    let _prop_b = JsPath::from_path(vec![b]);

    let c = "c".to_owned();
    let _prop_c = JsPath::from_path(&vec![c]);
}

#[test]
fn test_non_object_inputs_property_should_be_eq_null() {
    let non_objects: Vec<JsValue> = serde_json::from_value(json!([
        null,
        true,
        false,
        1,
        1.0,
        "a string",
        [],
        [1, 2, 3],
    ]))
    .unwrap();

    let prop_a = JsPath::from_path(vec!["a"]);

    for input in &non_objects {
        let value = prop_a.value(&input);
        assert!(prop_a.heq().are_eq(value.as_ref(), &json!(null)));
    }
}

#[test]
fn test_object_inputs_properties_eq() {
    let pairs = vec![
        (json!({}), json!(null)),
        (json!({"a": 1}), json!(1)),
        (json!({"a": {"b": 2}}), json!({"b": 2})),
    ];
    let prop_a = JsPath::from_path(vec!["a"]);

    for (ref input, ref expected_output) in &pairs {
        let actual_value = prop_a.value(&input);
        assert!(prop_a.heq().are_eq(actual_value.as_ref(), expected_output));
    }
}
