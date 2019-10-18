use json_utils::json::JsMap;
use json_utils::json::JsValue;

pub fn js_value_eq(left: &JsValue, right: &JsValue) -> bool {
    match (left, right) {
        (JsValue::Null, JsValue::Null) => true,
        (JsValue::Bool(left), JsValue::Bool(right)) => left == right,
        (JsValue::String(left), JsValue::String(right)) => left == right,
        (JsValue::Number(left), JsValue::Number(right)) => left == right,
        (JsValue::Array(left), JsValue::Array(right)) => js_array_eq(left, right),
        (JsValue::Object(left), JsValue::Object(right)) => js_object_eq(left, right),
        (_, _) => false,
    }
}

fn js_array_eq(left: &Vec<JsValue>, right: &Vec<JsValue>) -> bool {
    if left.len() == right.len() {
        left.iter()
            .zip(right.iter())
            .all(|(left, right)| js_value_eq(left, right))
    } else {
        false
    }
}

fn js_object_eq(left: &JsMap<String, JsValue>, right: &JsMap<String, JsValue>) -> bool {
    if left.len() == right.len() {
        left.keys().all(|key| {
            if let (Some(left), Some(right)) = (left.get(key), right.get(key)) {
                js_value_eq(left, right)
            } else {
                false
            }
        })
    } else {
        false
    }
}

#[test]
fn test_01_eq_neq() {
    assert!(js_value_eq(&json!(null), &json!(null)));
    assert!(!js_value_eq(&json!(null), &json!(false)));
    assert!(!js_value_eq(&json!(null), &json!(true)));

    assert!(js_value_eq(&json!(true), &json!(true)));
    assert!(js_value_eq(&json!(false), &json!(false)));
    assert!(!js_value_eq(&json!(true), &json!(false)));

    assert!(js_value_eq(&json!("a string"), &json!("a string")));
    assert!(!js_value_eq(&json!("a string"), &json!("another one")));

    assert!(js_value_eq(&json!(1), &json!(1)));
    assert!(!js_value_eq(&json!(1), &json!(2)));

    assert!(js_value_eq(&json!(-1), &json!(-1)));
    assert!(!js_value_eq(&json!(-1), &json!(1)));
    assert!(!js_value_eq(&json!(-1), &json!(-2)));

    assert!(js_value_eq(&json!(1.0), &json!(1.0)));
    assert!(js_value_eq(&json!(1.1), &json!(1.1)));
    assert!(!js_value_eq(&json!(1), &json!(1.1)));
    assert!(!js_value_eq(&json!(1.2), &json!(1.1)));

    assert!(js_value_eq(&json!([]), &json!([])));
    assert!(js_value_eq(&json!([1]), &json!([1])));
    assert!(js_value_eq(&json!([1, 2]), &json!([1, 2])));
    assert!(js_value_eq(&json!([1, 2, 3]), &json!([1, 2, 3])));

    assert!(!js_value_eq(&json!([]), &json!([0])));
    assert!(!js_value_eq(&json!([1]), &json!([1, 0])));
    assert!(!js_value_eq(&json!([1, 2]), &json!([1, 2, 0])));
    assert!(!js_value_eq(&json!([1, 2, 3]), &json!([1, 2, 3, 0])));

    assert!(!js_value_eq(&json!([1, 2]), &json!([2, 1])));
    assert!(!js_value_eq(&json!([1, 2, 3]), &json!([1, 3, 2])));

    assert!(js_value_eq(&json!({}), &json!({})));
    assert!(js_value_eq(&json!({"a": 1}), &json!({"a": 1})));
    assert!(js_value_eq(
        &json!({"a": 1, "b": 2}),
        &json!({"a": 1, "b": 2})
    ));
    assert!(js_value_eq(
        &json!({"a": 1, "b": 2}),
        &json!({"b": 2, "a": 1})
    ));

    assert!(!js_value_eq(&json!({"a": "1"}), &json!({"a": 1})));
    assert!(!js_value_eq(
        &json!({"a": 1, "b": "2"}),
        &json!({"a": 1, "b": 2})
    ));
    assert!(!js_value_eq(
        &json!({"a": 1, "b": "2"}),
        &json!({"b": 2, "a": 1})
    ));

    assert!(!js_value_eq(&json!({}), &json!({"_": 0})));
    assert!(!js_value_eq(&json!({"a": 1}), &json!({"a": 1, "_": 0})));
    assert!(!js_value_eq(
        &json!({"a": 1, "b": 2}),
        &json!({"a": 1, "b": 2, "_": 0})
    ));
    assert!(!js_value_eq(
        &json!({"a": 1, "b": 2}),
        &json!({"b": 2, "a": 1, "_": 0})
    ));
    assert!(!js_value_eq(
        &json!({"b": 2, "c": 1}),
        &json!({"a": 1, "b": 2})
    ));
}
