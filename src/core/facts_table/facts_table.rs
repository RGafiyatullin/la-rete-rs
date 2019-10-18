use std::collections::VecDeque;

use super::ArcDynFact;

#[derive(Clone)]
pub struct FactsRow<In> {
    pub(super) cells: VecDeque<ArcDynFact<In>>,
    pub(super) value: usize,
}

#[derive(Clone)]
pub struct FactsTable<In> {
    pub(super) rows: VecDeque<FactsRow<In>>,
}

impl<In> FactsRow<In> {
    pub fn new<I: IntoIterator<Item = ArcDynFact<In>>>(facts: I, value: usize) -> Self {
        let cells = facts.into_iter().collect();

        Self { cells, value }
    }
}

impl<In> FactsTable<In> {
    pub fn from_rows<I: IntoIterator<Item = FactsRow<In>>>(rows: I) -> Self {
        let rows = rows.into_iter().collect();

        Self { rows }
    }
}
