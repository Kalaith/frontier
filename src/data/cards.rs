//! Card data loading from JSON

use serde::{Deserialize, Serialize};
use crate::combat::{Card, CardEffect};

/// Raw card data from JSON (matches assets/cards.json structure)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardData {
    pub id: String,
    pub name: String,
    pub cost: i32,
    pub description: String,
    pub effects: Vec<CardEffect>,
}

impl CardData {
    /// Load all cards from the cards.json asset file
    pub fn load_all() -> Result<Vec<CardData>, String> {
        super::load_asset("cards.json")
    }
    
    /// Convert to a playable Card
    pub fn to_card(&self) -> Card {
        Card {
            id: self.id.clone(),
            name: self.name.clone(),
            cost: self.cost,
            description: self.description.clone(),
            effects: self.effects.clone(),
        }
    }
}

/// Load starter deck from JSON and convert to playable cards
pub fn load_starter_deck() -> Result<Vec<Card>, String> {
    let data = CardData::load_all()?;
    Ok(data.iter().map(|c| c.to_card()).collect())
}
