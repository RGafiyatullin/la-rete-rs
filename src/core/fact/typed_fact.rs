use std::marker::PhantomData;
use std::sync::Arc;

use super::Fact;
use super::FactsRow;
use super::FactsTable;
use super::HEqTable;
use super::Predicate;
use super::Property;
use super::Trie;
use super::TrieBuildFailure;
use super::TypedForkTable;

pub enum TypedFact<In, K, P: Property<In, K>> {
    PropertyIsEq {
        property: Arc<P>,
        value: Arc<K>,
        pd: PhantomData<In>,
    },
    PropertyIsNEq {
        property_id: String,
        property: Arc<P>,
        value: Arc<K>,
        pd: PhantomData<In>,
    },
    Predicate {
        property_id: String,
        property: Arc<P>,
        predicate: Arc<dyn Predicate<K>>,
        pd: PhantomData<In>,
    },
}

impl<In, K, P: Property<In, K>> TypedFact<In, K, P> {
    pub fn property_is_eq(property: P, value: K) -> Self {
        let property = Arc::new(property);
        let value = Arc::new(value);

        Self::PropertyIsEq {
            property,
            value,
            pd: PhantomData,
        }
    }

    pub fn property_is_neq(property: P, value: K) -> Self {
        let property = Arc::new(property);
        let value = Arc::new(value);
        let property_id = format!(
            "{}__neq__#{}",
            property.id(),
            property.heq().calc_hash(value.as_ref())
        );

        Self::PropertyIsNEq {
            property_id,
            property,
            value,
            pd: PhantomData,
        }
    }

    pub fn predicate<F: Predicate<K> + 'static>(property: P, predicate: F) -> Self {
        use rand::RngCore;

        let mut rng = rand::thread_rng();
        let rand = rng.next_u64();

        let property = Arc::new(property);
        let property_id = format!("TypedFact::Predicate__{}", rand);
        let predicate = Arc::new(predicate);

        Self::Predicate {
            property_id,
            property,
            predicate,
            pd: PhantomData,
        }
    }
}

impl<In: 'static, K: 'static, P: Property<In, K> + 'static> Fact<In> for TypedFact<In, K, P> {
    fn property_id(&self) -> String {
        match *self {
            Self::PropertyIsEq { ref property, .. } => property.id(),
            Self::PropertyIsNEq {
                ref property_id, ..
            } => property_id.clone(),
            Self::Predicate {
                ref property_id, ..
            } => property_id.clone(),
        }
    }

    fn value_hash(&self) -> Option<u64> {
        match *self {
            Self::PropertyIsEq {
                ref property,
                ref value,
                ..
            } => Some(property.heq().calc_hash(value)),
            Self::PropertyIsNEq { .. } => None,
            Self::Predicate { .. } => None,
        }
    }

    fn check(&self, input: &In) -> bool {
        match *self {
            Self::PropertyIsEq {
                ref property,
                ref value,
                ..
            } => property.heq().are_eq(property.value(input).as_ref(), value),

            Self::PropertyIsNEq {
                ref property,
                ref value,
                ..
            } => !property.heq().are_eq(property.value(input).as_ref(), value),

            Self::Predicate {
                ref property,
                ref predicate,
                ..
            } => predicate.apply(property.value(input).as_ref()),
        }
    }

    fn build_trie(
        &self,
        fact_rows: Vec<FactsRow<In>>,
        fallback: FactsTable<In>,
    ) -> Result<Trie<In>, TrieBuildFailure> {
        match *self {
            Self::PropertyIsEq { ref property, .. } => {
                build_trie_fork_eq(Arc::clone(property), fact_rows, fallback)
            }

            Self::PropertyIsNEq { ref property, .. } => {
                build_trie_fork_neq::<In, K, P>(Arc::clone(property), fact_rows, fallback)
            }

            Self::Predicate { .. } => build_trie_dumb_node(fact_rows, fallback),
        }
    }
    fn add_fact_to_group(
        &self,
        groups: Box<dyn std::any::Any>,
        _fact: Arc<dyn Fact<In>>,
        facts_row: FactsRow<In>,
    ) -> Box<dyn std::any::Any> {
        match *self {
            Self::PropertyIsEq { ref value, .. } => {
                let mut groups: Box<HEqTable<K, Vec<FactsRow<In>>>> = groups
                    .downcast()
                    .expect("Failed to downcast groups from std::any::Any");

                let _ = groups.insert(Arc::clone(value), Vec::new()).ok();
                groups
                    .lookup_mut(value.as_ref())
                    .expect("Failed to lookup a group that has just been inserted")
                    .push(facts_row);
                groups
            }

            _ => unreachable!("An attempt to insert a non-fork fact into the fact-groups table"),
        }
    }
}

fn build_trie_fork_eq<In: 'static, K: 'static, P: Property<In, K> + 'static>(
    property: Arc<P>,
    fact_rows: Vec<FactsRow<In>>,
    fallback: FactsTable<In>,
) -> Result<Trie<In>, TrieBuildFailure> {
    let groups = HEqTable::<_, Vec<FactsRow<In>>>::new(property.heq());
    let mut groups: Box<dyn std::any::Any> = Box::new(groups);

    for mut fact_row in fact_rows {
        let head_fact = fact_row
            .pop_fact()
            .expect("Expected fact_row to contain at least one fact");

        groups = Arc::clone(&head_fact).add_fact_to_group(groups, head_fact, fact_row);
    }

    let groups: Box<HEqTable<K, Vec<FactsRow<In>>>> = groups
        .downcast()
        .expect("Failed to downcast groups from std::any::Any");

    let eq_table_pairs = groups
        .into_iter()
        .map(|(key, rows)| (key, FactsTable::from_rows(rows)))
        .map(|(key, table)| table.into_trie().map(move |sub_trie| (key, sub_trie)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut eq_table = TypedForkTable::<In, K, P>::new(property);
    for (key, sub_trie) in eq_table_pairs {
        let () = eq_table.insert(key, sub_trie)?;
    }

    let eq_table = Box::new(eq_table);

    let fallback = fallback.into_trie()?;
    let fallback = Box::new(fallback);

    Ok(Trie::Fork { eq_table, fallback })
}

fn build_trie_fork_neq<In: 'static, K: 'static, P: Property<In, K> + 'static>(
    _property: Arc<P>,
    fact_rows: Vec<FactsRow<In>>,
    fallback: FactsTable<In>,
) -> Result<Trie<In>, TrieBuildFailure> {
    let mut head_acc = Vec::new();
    let mut tail_acc = Vec::new();

    for mut fact_row in fact_rows {
        let head_fact = fact_row
            .pop_fact()
            .expect("Expected fact_row to contain at least one fact");
        head_acc.push(head_fact);
        tail_acc.push(fact_row);
    }

    let fact = head_acc
        .into_iter()
        .next()
        .expect("Expected head_acc to contain at least one fact");
    let facts_table = FactsTable::from_rows(tail_acc);
    let sub_trie = facts_table.into_trie()?;
    let sub_trie = Box::new(sub_trie);

    let fallback = fallback.into_trie()?;
    let fallback = Box::new(fallback);

    let trie = Trie::Dumb {
        fact,
        then: sub_trie,
        fallback,
    };

    Ok(trie)
}

fn build_trie_dumb_node<In: 'static>(
    fact_rows: Vec<FactsRow<In>>,
    fallback: FactsTable<In>,
) -> Result<Trie<In>, TrieBuildFailure> {
    assert!(fact_rows.len() == 1);
    let mut head_row = fact_rows
        .into_iter()
        .next()
        .expect("Previous assertion failed (fact_rows.len() == 1)");
    let head_fact = head_row
        .pop_fact()
        .expect("Expected fact row to contain at least one fact");

    let facts_table = FactsTable::from_rows(vec![head_row]);
    let sub_trie = facts_table.into_trie()?;
    let sub_trie = Box::new(sub_trie);

    let fallback = fallback.into_trie()?;
    let fallback = Box::new(fallback);

    let trie = Trie::Dumb {
        fact: head_fact,
        then: sub_trie,
        fallback,
    };
    Ok(trie)
}
