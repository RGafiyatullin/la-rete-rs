use std::sync::Arc;

use super::fact::Fact;
use super::trie::Trie;
use super::TrieBuildFailure;

type ArcDynFact<In> = Arc<dyn Fact<In>>;

mod facts_table;
pub use facts_table::FactsRow;
pub use facts_table::FactsTable;

mod facts_table_stats;
pub use facts_table_stats::FactsTableStats;

mod facts_table_into_trie;
