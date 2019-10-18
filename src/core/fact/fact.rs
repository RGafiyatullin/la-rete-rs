use std::sync::Arc;

use super::FactsRow;
use super::FactsTable;
use super::Trie;
use super::TrieBuildFailure;

pub trait Fact<In> {
    fn property_id(&self) -> String;
    fn value_hash(&self) -> Option<u64>;
    fn check(&self, input: &In) -> bool;

    fn build_trie(
        &self,
        fact_rows: Vec<FactsRow<In>>,
        fallback: FactsTable<In>,
    ) -> Result<Trie<In>, TrieBuildFailure>;
    fn add_fact_to_group(
        &self,
        groups: Box<dyn std::any::Any>,
        fact: Arc<dyn Fact<In>>,
        facts_row: FactsRow<In>,
    ) -> Box<dyn std::any::Any>;
}
