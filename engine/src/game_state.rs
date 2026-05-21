use crate::{
    CardId, CardType, CardVec, Deck, DiscardPile, EntityId, Field, Hand, Lane, Player, Registry,
    SpawnSide, UnitState, UnitType,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct GameState {
    entity_number: u32,
    card_refs: HashMap<EntityId, CardId>,
    decks: HashMap<Player, Deck>,
    hands: HashMap<Player, Hand>,
    fields: HashMap<Player, Field>,
    discard_piles: HashMap<Player, DiscardPile>,
    units: HashMap<EntityId, UnitState>,
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

    pub fn spawn_unit(&mut self, registry: &Registry, card_entity: EntityId) -> Option<EntityId> {
        let card_id = self.find_card(card_entity)?;
        let unit_def = registry.get_card(card_id)?.as_unit()?;
        let unit_entity = self.create_entity();
        let unit_state = UnitState::new(unit_def);
        self.units.insert(unit_entity, unit_state);
        Some(unit_entity)
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

    pub fn assign_field(&mut self, player: Player, field: Field) {
        self.fields.insert(player, field);
    }

    pub fn assign_discard_pile(&mut self, player: Player, discard_pile: DiscardPile) {
        self.discard_piles.insert(player, discard_pile);
    }

    pub fn deck_mut(&mut self, player: Player) -> Option<&mut Deck> {
        self.decks.get_mut(&player)
    }

    pub fn hand_mut(&mut self, player: Player) -> Option<&mut Hand> {
        self.hands.get_mut(&player)
    }

    pub fn field_mut(&mut self, player: Player) -> Option<&mut Field> {
        self.fields.get_mut(&player)
    }

    pub fn discard_pile_mut(&mut self, player: Player) -> Option<&mut DiscardPile> {
        self.discard_piles.get_mut(&player)
    }

    pub fn draw_to_hand(&mut self, player: Player, number: usize) -> Option<Vec<EntityId>> {
        let drawn_cards = self.deck_mut(player)?.draw(number);
        self.hand_mut(player)?.add_many(drawn_cards.iter().copied());
        Some(drawn_cards)
    }

    pub fn play_card(
        &mut self,
        registry: &Registry,
        player: Player,
        card_entity: EntityId,
    ) -> Option<EntityId> {
        let card_id = self.find_card(card_entity)?;
        let card = registry.get_card(card_id)?;

        self.hand_mut(player)?.remove(card_entity)?;

        match &card.card_type {
            CardType::Unit(unit_def) => {
                let unit_entity = self.spawn_unit(registry, card_entity)?;
                self.play_unit(player, unit_entity, unit_def.unit_type)
            }
            CardType::Action { .. } => self.play_action(player, card_entity),
        }
    }

    fn play_unit(
        &mut self,
        player: Player,
        unit_entity: EntityId,
        unit_type: UnitType,
    ) -> Option<EntityId> {
        let lane = match unit_type {
            UnitType::Melee => Lane::Front,
            UnitType::Ranged => Lane::Back,
            UnitType::Legend => Lane::Front,
        };

        self.field_mut(player)?
            .add(unit_entity, lane, SpawnSide::Right);

        Some(unit_entity)
    }

    fn play_action(&mut self, player: Player, entity_id: EntityId) -> Option<EntityId> {
        self.hand_mut(player)?.remove(entity_id)?;
        Some(entity_id)
    }

    pub fn destroy_unit(&mut self, player: Player, entity_id: EntityId) -> Option<EntityId> {
        let destroyed_unit = self.field_mut(player)?.remove(entity_id)?;
        self.discard_pile_mut(player)?.add(entity_id);

        Some(destroyed_unit)
    }
}
