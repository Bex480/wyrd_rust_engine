use crate::EntityId;

pub struct Deck {
    cards: Vec<EntityId>,
}

impl Deck {
    pub fn new(cards: Vec<EntityId>) -> Self {
        Deck { cards }
    }

    pub fn draw(&mut self, number: usize) -> Vec<EntityId> {
        let n = number.min(self.card_count());
        self.cards.split_off(self.card_count() - n)
    }

    pub fn card_count(&self) -> usize {
        self.cards.len()
    }
}
