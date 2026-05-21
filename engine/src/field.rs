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

    fn remove_from(lane: &mut VecDeque<EntityId>, entity_id: EntityId) -> Option<EntityId> {
        let index = lane.iter().position(|id| *id == entity_id)?;
        lane.remove(index)
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Option<EntityId> {
        Self::remove_from(&mut self.frontline, entity_id)
            .or_else(|| Self::remove_from(&mut self.backline, entity_id))
    }

    pub fn unit_count_in_lane(&self, lane: Lane) -> usize {
        match lane {
            Lane::Front => self.frontline.len(),
            Lane::Back => self.backline.len(),
        }
    }

    pub fn unit_count_total(&self) -> usize {
        self.frontline.len() + self.backline.len()
    }
}
