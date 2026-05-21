use serde::{Deserialize, Serialize};

use crate::UnitDef;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct CardId(pub u32);

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CardDef {
    pub id: CardId,
    pub name: String,
    pub faction: Faction,
    pub tier: Tier,
    pub card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Unit(UnitDef),
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

impl CardDef {
    pub fn as_unit(&self) -> Option<&UnitDef> {
        match &self.card_type {
            CardType::Unit(unit_def) => Some(unit_def),
            _ => None,
        }
    }
}
