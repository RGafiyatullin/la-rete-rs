use std::sync::Arc;

use super::facts_table::FactsTableStats;
use super::ruleset::Ruleset;
use super::trie::Trie;
use super::TrieBuildFailure;

pub struct Matcher<In, Out> {
    trie: Trie<In>,
    values: Vec<Arc<Out>>,
}

impl<In, Out> Matcher<In, Out> {
    pub fn create(ruleset: Ruleset<In, Out>) -> Result<Self, TrieBuildFailure> {
        let (mut facts_table, values) = ruleset.into_table();
        let facts_table_stats = FactsTableStats::calculate(&facts_table);
        facts_table_stats.optimize_table(&mut facts_table);

        let trie = facts_table.into_trie()?;

        let matcher = Self { trie, values };

        Ok(matcher)
    }

    pub fn lookup<'a>(&'a self, input: &In) -> Option<&'a Out> {
        self.trie.lookup(input).map(|idx| self.values[idx].as_ref())
    }
}
