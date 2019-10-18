use std::sync::Arc;

use super::Fact;
use super::FactsRow;
use super::FactsTable;
use super::Rule;

pub struct Ruleset<In, Out> {
    rules: Vec<Rule<In, Out>>,
}

type TmpTable<In, Out> = Vec<(Vec<Arc<dyn Fact<In>>>, Arc<Out>)>;

impl<In, Out> Ruleset<In, Out> {
    pub fn new() -> Self {
        Ruleset { rules: vec![] }
    }

    field_get_ref!(rules, rules, Vec<Rule<In, Out>>);
    field_get_ref_mut!(rules, rules_mut, Vec<Rule<In, Out>>);
    field_update!(rules, with_rules, Vec<Rule<In, Out>>);

    pub fn into_table(self) -> (FactsTable<In>, Vec<Arc<Out>>) {
        let mut table = Vec::new();

        for rule in self.rules {
            let (facts_table, out) = rule.into_table();
            let out = Arc::new(out);
            let sub_table = facts_table
                .into_iter()
                .map(|facts| (facts, Arc::clone(&out)))
                .collect::<TmpTable<In, Out>>();

            table.extend(sub_table);
        }

        let mut rows = Vec::with_capacity(table.len());
        let mut outcomes = Vec::with_capacity(table.len());

        for (idx, (row, outcome)) in table.into_iter().enumerate() {
            let facts_row = FactsRow::new(row, idx);
            rows.push(facts_row);
            outcomes.push(outcome);
        }

        let facts_table = FactsTable::from_rows(rows);

        (facts_table, outcomes)
    }
}
