use crate::json::std as j;
use crate::prelude::*;
use json_utils::json::JsValue;

const TYPE: &'static str = "type";
const SUB_TYPE: &'static str = "sub-type";

const JS_NULL: JsValue = JsValue::Null;

const PRESENCE: &'static str = "presence";

const AVAILABLE: &'static str = "available";
const UNAVAILABLE: &'static str = "unavailable";
const SUBSCRIBE: &'static str = "subscribe";
const UNSUBSCRIBE: &'static str = "unsubscribe";
const SUBSCRIBED: &'static str = "subscribed";
const UNSUBSCRIBED: &'static str = "unsubscribed";

const PRESENCE_AVAILABILITY: &'static str = "presence:availability";
const PRESENCE_SUBSCRIPTION: &'static str = "presence:subscription";

#[test]
fn test() {
    let rules = vec![
        Rule::new(PRESENCE_AVAILABILITY).with_filter(
            Filter::fact(j::str::eq(vec![TYPE], PRESENCE)).and(
                Filter::fact(j::eq(vec![SUB_TYPE], JS_NULL))
                    .or(Filter::fact(j::str::eq(vec![SUB_TYPE], AVAILABLE)))
                    .or(Filter::fact(j::str::eq(vec![SUB_TYPE], UNAVAILABLE))),
            ),
        ),
        Rule::new(PRESENCE_SUBSCRIPTION).with_filter(
            Filter::fact(j::str::eq(vec![TYPE], PRESENCE))
                .and(Filter::fact(j::str::eq(vec![SUB_TYPE], SUBSCRIBE)))
                .or(Filter::fact(j::str::eq(vec![SUB_TYPE], UNSUBSCRIBE)))
                .or(Filter::fact(j::str::eq(vec![SUB_TYPE], SUBSCRIBED)))
                .or(Filter::fact(j::str::eq(vec![SUB_TYPE], UNSUBSCRIBED))),
        ),
    ];
    let ruleset = Ruleset::new().with_rules(rules);
    let matcher = Matcher::create(ruleset).expect("Failed to compile ruleset");

    assert_eq!(matcher.lookup(&json!({})), None);
    assert_eq!(
        matcher.lookup(&json!({
            TYPE: PRESENCE,
            SUB_TYPE: "wrong"
        })),
        None
    );
    assert_eq!(
        matcher.lookup(&json!({ TYPE: PRESENCE })),
        Some(&PRESENCE_AVAILABILITY)
    );
    assert_eq!(
        matcher.lookup(&json!({TYPE: PRESENCE, SUB_TYPE: AVAILABLE})),
        Some(&PRESENCE_AVAILABILITY)
    );
    assert_eq!(
        matcher.lookup(&json!({TYPE: PRESENCE, SUB_TYPE: UNAVAILABLE})),
        Some(&PRESENCE_AVAILABILITY)
    );
    assert_eq!(
        matcher.lookup(&json!({ TYPE: PRESENCE, SUB_TYPE: SUBSCRIBE })),
        Some(&PRESENCE_SUBSCRIPTION)
    );
    assert_eq!(
        matcher.lookup(&json!({ TYPE: PRESENCE, SUB_TYPE: UNSUBSCRIBE })),
        Some(&PRESENCE_SUBSCRIPTION)
    );
    assert_eq!(
        matcher.lookup(&json!({ TYPE: PRESENCE, SUB_TYPE: UNSUBSCRIBE })),
        Some(&PRESENCE_SUBSCRIPTION)
    );
    assert_eq!(
        matcher.lookup(&json!({ TYPE: PRESENCE, SUB_TYPE: UNSUBSCRIBED })),
        Some(&PRESENCE_SUBSCRIPTION)
    );
}
