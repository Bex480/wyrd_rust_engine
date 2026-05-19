use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct CardId(pub u32);

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CardDef {
    pub id: CardId,
    pub name: String,
    pub faction: Faction,
    pub card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Unit {
        health: i32,
        armour: i32,
        essence_yield: i32,
        tier: Tier,
        unit_type: UnitType,
        abilities: Vec<i32>,
    },
    Action {
        cost: i32,
        tier: Tier,
        abilities: Vec<i32>,
    },
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Faction {
    Nature,
    Crimson,
    Sun,
    Myridian,
    Guild,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Tier {
    Low,
    Mid,
    High,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum UnitType {
    Legend,
    Melee,
    Ranged,
}
