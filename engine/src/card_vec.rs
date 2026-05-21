use crate::EntityId;

pub trait CardVec {
    fn cards(&self) -> &Vec<EntityId>;
    fn cards_mut(&mut self) -> &mut Vec<EntityId>;

    fn card_count(&self) -> usize {
        self.cards().len()
    }

    fn add(&mut self, entity_id: EntityId) {
        self.cards_mut().push(entity_id);
    }

    fn add_many<I: IntoIterator<Item = EntityId>>(&mut self, cards: I) {
        self.cards_mut().extend(cards);
    }

    fn remove(&mut self, entity_id: EntityId) -> Option<EntityId> {
        let index = self.cards_mut().iter().position(|id| *id == entity_id)?;
        Some(self.cards_mut().remove(index))
    }
}
