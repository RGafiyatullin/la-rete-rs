use super::facts_table::FactsRow;
use super::facts_table::FactsTable;
use super::heq::HEqTable;
use super::property::Property;
use super::trie::Trie;
use super::trie::TypedForkTable;
use super::TrieBuildFailure;

mod fact;
pub use fact::Fact;

mod typed_fact;
pub use typed_fact::TypedFact;

mod predicate;
pub use predicate::Predicate;
