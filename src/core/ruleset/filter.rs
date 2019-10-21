use std::sync::Arc;

use super::Fact;

pub enum Filter<In> {
    Fact(Box<dyn Fact<In>>),
    And(Vec<Self>),
    Or(Vec<Self>),
}

impl<In> Filter<In> {
    pub fn empty() -> Self {
        Self::And(vec![])
    }

    pub fn fact<F: Fact<In> + 'static>(fact: F) -> Self {
        Filter::Fact(Box::new(fact))
    }

    pub fn and(self, other: Self) -> Self {
        Filter::And(vec![self, other])
    }

    pub fn or(self, other: Self) -> Self {
        Filter::Or(vec![self, other])
    }

    pub fn into_table(self) -> Vec<Vec<Arc<dyn Fact<In>>>> {
        into_table(vec![vec![]], self)
    }
}

fn into_table<In>(
    mut acc: Vec<Vec<Arc<dyn Fact<In>>>>,
    filter: Filter<In>,
) -> Vec<Vec<Arc<dyn Fact<In>>>> {
    match filter {
        Filter::Fact(fact) => {
            let fact = Arc::from(fact);

            for row in &mut acc {
                row.push(Arc::clone(&fact));
            }

            acc
        }

        Filter::And(filters) => filters
            .into_iter()
            .fold(acc, |acc, filter| into_table(acc, filter)),

        Filter::Or(filters) => {
            let mut out = Vec::new();

            for filter in filters {
                let alt = into_table(acc.clone(), filter);
                out.extend(alt);
            }

            out
        }
    }
}
