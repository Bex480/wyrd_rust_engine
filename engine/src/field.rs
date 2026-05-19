use std::collections::VecDeque;

use crate::EntityId;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Field {
    pub frontline: VecDeque<EntityId>,
    pub backline: VecDeque<EntityId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    Front,
    Back,
}

impl Field {
    pub fn new() -> Self {
        Self {
            frontline: VecDeque::new(),
            backline: VecDeque::new(),
        }
    }

    pub fn add(&mut self, entity_id: EntityId, lane: Lane, side: SpawnSide) {
        let selected_lane = match lane {
            Lane::Front => &mut self.frontline,
            Lane::Back => &mut self.backline,
        };

        match side {
            SpawnSide::Left => selected_lane.push_front(entity_id),
            SpawnSide::Right => selected_lane.push_back(entity_id),
        }
    }

    pub fn unit_count_frontline(&self) -> usize {
        self.frontline.len()
    }

    pub fn unit_count_backline(&self) -> usize {
        self.backline.len()
    }

    pub fn unit_count_total(&self) -> usize {
        self.frontline.len() + self.backline.len()
    }
}
