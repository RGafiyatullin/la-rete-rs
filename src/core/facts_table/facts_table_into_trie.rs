use super::ArcDynFact;
use super::FactsRow;
use super::FactsTable;
use super::Trie;
use super::TrieBuildFailure;

impl<In> FactsTable<In> {
    pub fn into_trie(mut self) -> Result<Trie<In>, TrieBuildFailure> {
        if let Some(head_row) = self.peek_row() {
            if let Some(property_id) = head_row.peek_fact().map(|f| f.property_id()) {
                let mut acc = Vec::new();
                while self
                    .peek_row()
                    .and_then(|head_row| head_row.peek_fact().map(|f| f.property_id()))
                    .as_ref()
                    == Some(&property_id)
                {
                    acc.push(
                        self.pop_row()
                            .expect("Got None whereas it is expected to be Some(_) #1"),
                    );
                }
                let aux_fact = acc[0]
                    .peek_fact()
                    .expect("Got None whereas it is expected to be Some(_) #2");
                aux_fact.build_trie(acc, self)
            } else {
                Ok(Trie::Value(head_row.value))
            }
        } else {
            Ok(Trie::Nil)
        }
    }

    pub fn pop_row(&mut self) -> Option<FactsRow<In>> {
        self.rows.pop_front()
    }

    pub fn peek_row(&self) -> Option<&FactsRow<In>> {
        self.rows.get(0)
    }
}

impl<In> FactsRow<In> {
    pub fn peek_fact(&self) -> Option<ArcDynFact<In>> {
        self.cells.get(0).cloned()
    }

    pub fn pop_fact(&mut self) -> Option<ArcDynFact<In>> {
        self.cells.pop_front()
    }
    pub fn push_fact(&mut self, fact: ArcDynFact<In>) -> () {
        self.cells.push_front(fact)
    }
}
