use engine::{CardDef, CardId, EntityId, GameState, PlayerId};

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
