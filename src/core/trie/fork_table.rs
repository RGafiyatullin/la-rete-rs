use std::sync::Arc;

use super::HEqTable;
use super::Property;
use super::Trie;

use super::TrieBuildFailure;

pub trait ForkTable<In> {
    fn lookup<'a>(&'a self, input: &In) -> Option<&'a Trie<In>>;
}

pub struct TypedForkTable<In, K, P: Property<In, K>> {
    property: Arc<P>,
    table: HEqTable<K, Trie<In>>,
}

impl<In, K, P: Property<In, K>> TypedForkTable<In, K, P> {
    pub fn new(property: Arc<P>) -> Self {
        let heq = property.heq();
        Self {
            property,
            table: HEqTable::new(heq),
        }
    }

    pub fn insert(&mut self, key: Arc<K>, sub_trie: Trie<In>) -> Result<(), TrieBuildFailure> {
        let _ = self.table.insert(key, sub_trie).ok(); // TODO: do not be silent here. Introduce BuildContext in order to put warnings there
        Ok(())
    }
}

impl<In, K, P: Property<In, K>> ForkTable<In> for TypedForkTable<In, K, P> {
    fn lookup<'a>(&'a self, input: &In) -> Option<&'a Trie<In>> {
        let key = self.property.value(input);
        self.table.lookup(key.as_ref())
    }
}
