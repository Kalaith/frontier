//! Card data loading from JSON

use serde::{Deserialize, Serialize};
use crate::combat::{Card, CardEffect, CardClass};

/// Raw card data from JSON (matches assets/cards.json structure)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardData {
    pub id: String,
    pub name: String,
    pub cost: i32,
    pub description: String,
    pub effects: Vec<CardEffect>,
    #[serde(default)]
    pub image_path: Option<String>,
    #[serde(default)]
    pub class: CardClass,
}

impl CardData {
    /// Load all cards from the cards.json asset file
    pub fn load_all() -> Result<Vec<CardData>, String> {
        crate::load_asset!("cards.json", Vec<CardData>)
    }
    
    /// Check if this card can be used by the given class name
    pub fn class_matches(&self, class_name: &str) -> bool {
        self.class.matches(class_name)
    }
    
    /// Convert to a playable Card
    pub fn to_card(&self) -> Card {
        Card {
            id: self.id.clone(),
            name: self.name.clone(),
            cost: self.cost,
            description: self.description.clone(),
            effects: self.effects.clone(),
            image_path: self.image_path.clone(),
            class: self.class.clone(),
        }
    }
}

/// Load starter deck from JSON and convert to playable cards
#[allow(dead_code)]
pub fn load_starter_deck() -> Result<Vec<Card>, String> {
    let data = CardData::load_all()?;
    Ok(data.iter().map(|c| c.to_card()).collect())
}

