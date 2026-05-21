use crate::{CardVec, EntityId};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DiscardPile {
    cards: Vec<EntityId>,
}

impl DiscardPile {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
}

impl Default for DiscardPile {
    fn default() -> Self {
        Self::new()
    }
}

impl CardVec for DiscardPile {
    fn cards(&self) -> &Vec<EntityId> {
        &self.cards
    }

    fn cards_mut(&mut self) -> &mut Vec<EntityId> {
        &mut self.cards
    }
}
