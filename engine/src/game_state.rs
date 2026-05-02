use crate::{CardId, Deck, EntityId, PlayerId};
use std::collections::HashMap;

pub struct GameState {
    entity_number: u32,
    card_refs: HashMap<EntityId, CardId>,
    decks: HashMap<PlayerId, Deck>,
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

    pub fn assign_deck(&mut self, player: PlayerId, deck: Deck) {
        self.decks.insert(player, deck);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            entity_number: 0,
            card_refs: HashMap::new(),
            decks: HashMap::new(),
        }
    }
}
