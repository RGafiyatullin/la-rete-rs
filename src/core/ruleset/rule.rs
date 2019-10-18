use std::sync::Arc;

use super::Fact;
use super::Filter;

pub struct Rule<In, Out> {
    filter: Filter<In>,
    value: Out,
}

impl<In, Out> Rule<In, Out> {
    field_get_ref!(filter, filter, Filter<In>);
    field_get_ref_mut!(filter, filter_mut, Filter<In>);
    field_update!(filter, with_filter, Filter<In>);

    pub fn new(value: Out) -> Self {
        let filter = Filter::empty();
        Self { filter, value }
    }

    pub fn into_table(self) -> (Vec<Vec<Arc<dyn Fact<In>>>>, Out) {
        let value = self.value;
        let table = self.filter.into_table();
        (table, value)
    }
}
