use std::sync::Arc;

use super::Fact;
use super::ForkTable;

type ArcDynFact<In> = Arc<dyn Fact<In>>;

pub enum Trie<In> {
    Nil,
    Value(usize),
    Dumb {
        fact: ArcDynFact<In>,
        then: Box<Trie<In>>,
        fallback: Box<Trie<In>>,
    },
    Fork {
        eq_table: Box<dyn ForkTable<In>>,
        fallback: Box<Trie<In>>,
    },
}

impl<In> Trie<In> {
    pub fn lookup(&self, input: &In) -> Option<usize> {
        match *self {
            Self::Nil => None,
            Self::Value(value) => Some(value),
            Self::Dumb {
                ref fact,
                ref then,
                ref fallback,
            } => {
                if fact.check(input) {
                    then.lookup(input)
                } else {
                    fallback.lookup(input)
                }
            }
            Self::Fork {
                ref eq_table,
                ref fallback,
            } => {
                if let Some(sub_trie) = eq_table.lookup(input) {
                    sub_trie.lookup(input)
                } else {
                    fallback.lookup(input)
                }
            }
        }
    }
}
