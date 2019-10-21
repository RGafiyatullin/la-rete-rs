use crate::json::std as j;
use crate::prelude::*;

const LEN_EQ_0: &'static str = "len-eq-0";
const LEN_EQ_5: &'static str = "len-eq-5";

const LEN_GTE_7: &'static str = "len-gte-4";
const LEN_GT_5: &'static str = "len-gt-5";

const LEN_LTE_2: &'static str = "len-lte-2";
const LEN_LT_4: &'static str = "len-lt-4";

#[test]
fn test() {
    let rules = vec![
        Rule::new(LEN_EQ_0).with_filter(Filter::fact(j::list::len_eq(vec!["list"], 0))),
        Rule::new(LEN_EQ_5).with_filter(Filter::fact(j::list::len_eq(vec!["list"], 5))),
        Rule::new(LEN_GTE_7).with_filter(Filter::fact(j::list::len_gte(vec!["list"], 7))),
        Rule::new(LEN_GT_5).with_filter(Filter::fact(j::list::len_gt(vec!["list"], 5))),
        Rule::new(LEN_LTE_2).with_filter(Filter::fact(j::list::len_lte(vec!["list"], 2))),
        Rule::new(LEN_LT_4).with_filter(Filter::fact(j::list::len_lt(vec!["list"], 4))),
    ];
    let ruleset = Ruleset::new().with_rules(rules);
    let matcher = Matcher::create(ruleset).expect("Failed to compile ruleset");

    assert_eq!(matcher.lookup(&json!({})), None);
    assert_eq!(
        matcher.lookup(&json!({
            "list": []
        })),
        Some(&LEN_EQ_0)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1]
        })),
        Some(&LEN_LTE_2)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2]
        })),
        Some(&LEN_LTE_2)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3]
        })),
        Some(&LEN_LT_4)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4]
        })),
        None
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4,5]
        })),
        Some(&LEN_EQ_5)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4,5,6]
        })),
        Some(&LEN_GT_5)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4,5,6,7]
        })),
        Some(&LEN_GTE_7)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4,5,6,7,8]
        })),
        Some(&LEN_GTE_7)
    );
    assert_eq!(
        matcher.lookup(&json!({
            "list": [1,2,3,4,5,6,7,8,9]
        })),
        Some(&LEN_GTE_7)
    );
}
