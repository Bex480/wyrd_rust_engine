use crate::EntityId;

pub struct Hand {
    cards: Vec<EntityId>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn with_cards(cards: Vec<EntityId>) -> Self {
        Self { cards }
    }

    pub fn add(&mut self, entity_id: EntityId) {
        self.cards.push(entity_id);
    }

    pub fn add_many<I: IntoIterator<Item = EntityId>>(&mut self, cards: I) {
        self.cards.extend(cards);
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Option<EntityId> {
        let card_index = self.cards.iter().position(|id| *id == entity_id)?;
        Some(self.cards.remove(card_index))
    }

    pub fn card_count(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}
