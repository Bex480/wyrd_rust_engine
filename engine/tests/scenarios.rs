use engine::{CardId, EntityId, PlayerId};

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
