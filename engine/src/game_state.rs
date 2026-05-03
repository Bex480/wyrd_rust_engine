use crate::{CardId, Deck, EntityId, Hand, Player, Registry};
use std::collections::HashMap;

pub struct GameState {
    entity_number: u32,
    card_refs: HashMap<EntityId, CardId>,
    decks: HashMap<Player, Deck>,
    hands: HashMap<Player, Hand>,
}

impl GameState {
    fn create_entity(&mut self) -> EntityId {
        self.entity_number += 1;
        EntityId(self.entity_number)
    }

    pub fn spawn_card(&mut self, registry: &Registry, def_id: CardId) -> Option<EntityId> {
        registry.get_card(def_id)?;
        let entity_id = self.create_entity();
        self.card_refs.insert(entity_id, def_id);
        Some(entity_id)
    }

    pub fn find_card(&self, entity_id: EntityId) -> Option<CardId> {
        self.card_refs.get(&entity_id).copied()
    }

    pub fn assign_deck(&mut self, player: Player, deck: Deck) {
        self.decks.insert(player, deck);
    }

    pub fn assign_hand(&mut self, player: Player, hand: Hand) {
        self.hands.insert(player, hand);
    }

    pub fn deck_mut(&mut self, player: Player) -> Option<&mut Deck> {
        self.decks.get_mut(&player)
    }

    pub fn hand_mut(&mut self, player: Player) -> Option<&mut Hand> {
        self.hands.get_mut(&player)
    }

    pub fn draw_to_hand(&mut self, player: Player, number: usize) -> Option<Vec<EntityId>> {
        let drawn_cards = self.deck_mut(player)?.draw(number);
        self.hand_mut(player)?.add_many(drawn_cards.iter().copied());
        Some(drawn_cards)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            entity_number: 0,
            card_refs: HashMap::new(),
            decks: HashMap::new(),
            hands: HashMap::new(),
        }
    }
}
