use crate::CardDef;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse RON: {0}")]
    Parse(#[from] ron::error::SpannedError),
}

pub fn load_cards(path: &Path) -> Result<Vec<CardDef>, LoadError> {
    let contents = std::fs::read_to_string(path)?;
    let cards = ron::from_str(&contents)?;
    Ok(cards)
}
