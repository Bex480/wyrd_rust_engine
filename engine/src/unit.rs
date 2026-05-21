use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct UnitState {
    max_health: i32,
    current_health: i32,
    current_armour: i32,
    current_essence_yield: i32,
    statuses: Vec<Status>,
    current_abilities: Vec<i32>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct UnitDef {
    pub health: i32,
    pub armour: i32,
    pub essence_yield: i32,
    pub unit_type: UnitType,
    pub abilities: Vec<i32>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum UnitType {
    Legend,
    Melee,
    Ranged,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Status {
    Frozen,
}

impl UnitState {
    pub fn new(unit_def: &UnitDef) -> Self {
        Self {
            max_health: unit_def.health,
            current_health: unit_def.health,
            current_armour: unit_def.armour,
            current_essence_yield: unit_def.essence_yield,
            statuses: Vec::new(),
            current_abilities: unit_def.abilities.clone(),
        }
    }
}
