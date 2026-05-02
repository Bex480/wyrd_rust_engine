use crate::{load_cards, CardDef, CardId, LoadError};
use std::{collections::HashMap, path::Path};

pub struct Registry {
    cards: HashMap<CardId, CardDef>,
}

impl Registry {
    pub fn new(path: &Path) -> Result<Self, LoadError> {
        let loaded_cards = load_cards(path)?;
        let cards = loaded_cards.into_iter().map(|c| (c.id, c)).collect();

        Ok(Self { cards })
    }

    pub fn load_default() -> Result<Self, LoadError> {
        Self::new(Path::new("data/cards.ron"))
    }

    pub fn get_card(&self, card_id: CardId) -> Option<&CardDef> {
        self.cards.get(&card_id)
    }
}
