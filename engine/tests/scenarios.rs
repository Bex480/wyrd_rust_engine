use std::path::Path;

use engine::{
    CardDef, CardId, CardType, Deck, EntityId, Faction, Field, GameState, Hand, Lane, Player,
    Registry, SpawnSide, Tier, UnitType,
};

#[test]
fn player_id_other_swap_players() {
    assert_eq!(Player::P1.other(), Player::P2);
    assert_eq!(Player::P2.other(), Player::P1);
}

#[test]
fn entity_ids_compare_by_inner_value() {
    assert_eq!(EntityId(5), EntityId(5));
    assert_ne!(EntityId(5), EntityId(6));
}

#[test]
fn card_ids_compare_by_inner_value() {
    assert_eq!(CardId(5), CardId(5));
    assert_ne!(CardId(5), CardId(6));
}

#[test]
fn card_def_compare_by_field_value() {
    let def = CardDef {
        id: CardId(1),
        name: "Tundrakii Matron".to_string(),
        faction: Faction::Myridian,
        card_type: CardType::Unit {
            health: 4,
            armour: 0,
            essence_yield: 1,
            tier: Tier::Mid,
            unit_type: UnitType::Ranged,
            abilities: vec![],
        },
    };
    let def_copy = CardDef {
        id: CardId(1),
        name: "Tundrakii Matron".to_string(),
        faction: Faction::Myridian,
        card_type: CardType::Unit {
            health: 4,
            armour: 0,
            essence_yield: 1,
            tier: Tier::Mid,
            unit_type: UnitType::Ranged,
            abilities: vec![],
        },
    };
    let def_diff = CardDef {
        id: CardId(2),
        name: "Tundrakii Huntress".to_string(),
        faction: Faction::Myridian,
        card_type: CardType::Unit {
            health: 3,
            armour: 0,
            essence_yield: 1,
            tier: Tier::Low,
            unit_type: UnitType::Ranged,
            abilities: vec![],
        },
    };

    assert_eq!(def, def_copy);
    assert_ne!(def, def_diff);
}

#[test]
fn card_entity_id_check() {
    let mut game_state = GameState::default();
    let registry = Registry::load_default().expect("load cards");
    let entity_id_1 = game_state
        .spawn_card(&registry, CardId(1))
        .expect("spawn card 1");
    let entity_id_2 = game_state
        .spawn_card(&registry, CardId(2))
        .expect("spawn card 2");
    let card_id = game_state.find_card(entity_id_1);

    assert_ne!(entity_id_1, entity_id_2);
    assert_eq!(card_id, Some(CardId(1)));
}

#[test]
fn load_cards_from_ron_file() {
    let path = Path::new("data/cards.ron");
    let cards = engine::load_cards(path).expect("loaded cards");

    assert!(!cards.is_empty(), "Should load at least one card");
}

#[test]
fn fetch_card_from_default_registry() {
    let registry = Registry::load_default().expect("load cards");
    let card = registry.get_card(CardId(1)).expect("get card with Id 1");

    assert_eq!(card.id, CardId(1));
}

#[test]
fn create_and_populate_deck() {
    let mut game_state = GameState::default();
    let registry = Registry::load_default().expect("load cards");
    let card_1 = game_state
        .spawn_card(&registry, CardId(1))
        .expect("spawn card 1");
    let card_2 = game_state
        .spawn_card(&registry, CardId(2))
        .expect("spawn card 2");
    let mut deck = Deck::new(vec![card_1, card_2]);

    assert_eq!(deck.card_count(), 2);
    let drawn = deck.draw(1);
    assert_eq!(drawn.len(), 1);
    assert_eq!(deck.card_count(), 1);
}

#[test]
fn draw_card_from_deck() {
    let registry = Registry::load_default().expect("load cards");
    let mut game_state = GameState::default();
    let card_1 = game_state
        .spawn_card(&registry, CardId(1))
        .expect("spawn card 1");
    let card_2 = game_state
        .spawn_card(&registry, CardId(2))
        .expect("spawn card 2");
    let deck = Deck::new(vec![card_1, card_2]);

    game_state.assign_deck(Player::P1, deck);
    game_state.assign_hand(Player::P1, Hand::new());

    game_state
        .draw_to_hand(Player::P1, 1)
        .expect("drawn one 1 card");

    let deck_after = game_state.deck_mut(Player::P1).expect("deck after draw");
    assert_eq!(deck_after.card_count(), 1);

    let hand_after = game_state.hand_mut(Player::P1).expect("hand after draw");
    assert_eq!(hand_after.card_count(), 1);
}

#[test]
fn field_creation_and_unit_spawning() {
    let registry = Registry::load_default().expect("Load cards!");
    let mut game_state = GameState::default();
    let card_1 = game_state
        .spawn_card(&registry, CardId(1))
        .expect("spawn card 1");

    game_state.assign_field(Player::P1, Field::new());
    let field = game_state.field_mut(Player::P1).expect("field creation!");

    field.add(card_1, Lane::Back, SpawnSide::Right);

    assert_eq!(field.unit_count_in_lane(Lane::Back), 1);
}

#[test]
fn play_card_from_hand() {
    let registry = Registry::load_default().expect("load cards");
    let mut game_state = GameState::default();
    let card_1 = game_state.spawn_card(&registry, CardId(1)).expect("spawn");

    game_state.assign_deck(Player::P1, Deck::new(vec![card_1]));
    game_state.assign_hand(Player::P1, Hand::new());
    game_state.assign_field(Player::P1, Field::new());

    assert_eq!(game_state.deck_mut(Player::P1).unwrap().card_count(), 1);

    game_state.draw_to_hand(Player::P1, 1).expect("draw");
    assert_eq!(game_state.deck_mut(Player::P1).unwrap().card_count(), 0);
    assert_eq!(game_state.hand_mut(Player::P1).unwrap().card_count(), 1);

    game_state
        .play_card(&registry, Player::P1, card_1)
        .expect("play");
    assert_eq!(game_state.hand_mut(Player::P1).unwrap().card_count(), 0);
    assert_eq!(
        game_state.field_mut(Player::P1).unwrap().unit_count_total(),
        1
    );
}
