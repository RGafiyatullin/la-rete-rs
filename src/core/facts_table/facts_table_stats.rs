use std::collections::HashMap;
use std::collections::HashSet;

use super::FactsTable;

pub struct FactsTableStats {
    selectivity: HashMap<String, usize>,
}

impl FactsTableStats {
    pub fn calculate<In>(facts_table: &FactsTable<In>) -> Self {
        let mut selectivity = HashMap::<String, HashSet<u64>>::new();

        for row in &facts_table.rows {
            for cell in &row.cells {
                let prop_id = cell.property_id();
                if let Some(value_hash) = cell.value_hash() {
                    let mut hashes = selectivity.remove(&prop_id).unwrap_or(HashSet::new());
                    hashes.insert(value_hash);
                    selectivity.insert(prop_id, hashes);
                }
            }
        }

        let selectivity = selectivity
            .into_iter()
            .map(|(k, hs)| (k, hs.len()))
            .collect();
        Self { selectivity }
    }

    pub fn optimize_table<In>(&self, facts_table: &mut FactsTable<In>) -> () {
        for row in &mut facts_table.rows {
            let mut cells = row.cells.iter().cloned().collect::<Vec<_>>();
            cells.sort_by(|left, right| {
                let left = self.selectivity.get(&left.property_id()).unwrap_or(&0);
                let right = self.selectivity.get(&right.property_id()).unwrap_or(&0);

                right.cmp(left)
            });
            row.cells = cells.into_iter().collect();
        }
    }
}
