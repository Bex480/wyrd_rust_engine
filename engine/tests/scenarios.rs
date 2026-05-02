use std::path::Path;

use engine::{CardDef, CardId, Deck, EntityId, GameState, PlayerId, Registry};

#[test]
fn player_id_other_swap_players() {
    assert_eq!(PlayerId::P1.other(), PlayerId::P2);
    assert_eq!(PlayerId::P2.other(), PlayerId::P1);
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
        health: 4,
    };
    let def_copy = CardDef {
        id: CardId(1),
        name: "Tundrakii Matron".to_string(),
        health: 4,
    };
    let def_diff = CardDef {
        id: CardId(2),
        name: "Tundrakii Huntress".to_string(),
        health: 3,
    };

    assert_eq!(def, def_copy);
    assert_ne!(def, def_diff);
}

#[test]
fn card_entity_id_check() {
    let mut game_state = GameState::default();
    let entity_id_1 = game_state.spawn_card(CardId(33));
    let entity_id_2 = game_state.spawn_card(CardId(37));
    let card_id = game_state.find_card(entity_id_1);

    assert_ne!(entity_id_1, entity_id_2);
    assert_eq!(card_id, Some(CardId(33)));
}

#[test]
fn load_cards_from_ron_file() {
    let path = Path::new("data/cards.ron");
    let cards = engine::load_cards(path).expect("loaded cards");

    assert!(!cards.is_empty(), "Should load at least one card");
}

#[test]
fn fetch_card_from_default_registry() {
    let registry = Registry::load_default().unwrap();
    let card = registry.get_card(CardId(1)).unwrap();

    assert_eq!(card.id, CardId(1));
}

#[test]
fn create_and_populate_deck() {
    let mut game_state = GameState::default();
    let card_1 = game_state.spawn_card(CardId(1));
    let card_2 = game_state.spawn_card(CardId(2));
    let mut deck = Deck::new(vec![card_1, card_2]);

    assert_eq!(deck.card_count(), 2);
    let drawn = deck.draw(1);
    assert_eq!(drawn.len(), 1);
    assert_eq!(deck.card_count(), 1);
}
