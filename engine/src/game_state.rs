use crate::{CardId, EntityId};
use std::collections::HashMap;

pub struct GameState {
    entity_number: u32,
    card_refs: HashMap<EntityId, CardId>,
}

impl GameState {
    fn create_entity(&mut self) -> EntityId {
        self.entity_number += 1;
        EntityId(self.entity_number)
    }

    pub fn spawn_card(&mut self, def_id: CardId) -> EntityId {
        let entity_id = self.create_entity();
        self.card_refs.insert(entity_id, def_id);
        entity_id
    }

    pub fn find_card(&self, entity_id: EntityId) -> Option<CardId> {
        self.card_refs.get(&entity_id).copied()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            entity_number: 0,
            card_refs: HashMap::new(),
        }
    }
}
