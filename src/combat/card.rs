//! Card definitions and starter deck

use serde::{Deserialize, Serialize};
use super::CardEffect;

/// IDs of cards in the starter deck (drawn from JSON data)
pub const STARTER_DECK_IDS: &[&str] = &[
    "strike",
    "strike",
    "guard", 
    "guard",
    "desperate_swing",
    "recenter",
];

/// A playable card
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub cost: i32,
    pub description: String,
    pub effects: Vec<CardEffect>,
    #[serde(default)]
    pub image_path: Option<String>,
}

impl Card {
    /// Check if this card is an attack (deals damage)
    pub fn is_attack(&self) -> bool {
        self.effects.iter().any(|e| matches!(e, 
            CardEffect::Damage(_) | 
            CardEffect::DamageIfNoBlock { .. } | 
            CardEffect::DamageIfLowHp { .. } |
            CardEffect::DamageIfEnemyActed { .. }
        ))
    }
    
    /// Load the starter hand from JSON data
    /// Returns a hand of 5 cards drawn from starter deck
    pub fn starter_hand() -> Vec<Card> {
        Self::load_starter_deck()
            .into_iter()
            .take(5)
            .collect()
    }
    
    /// Load full starter deck from JSON
    pub fn load_starter_deck() -> Vec<Card> {
        match crate::data::cards::CardData::load_all() {
            Ok(all_cards) => {
                STARTER_DECK_IDS
                    .iter()
                    .filter_map(|id| {
                        all_cards.iter()
                            .find(|c| c.id == *id)
                            .map(|c| c.to_card())
                    })
                    .collect()
            }
            Err(e) => {
                eprintln!("Failed to load cards from JSON: {}. Using fallback.", e);
                Self::fallback_starter_hand()
            }
        }
    }
    
    /// Fallback hand if JSON loading fails
    fn fallback_starter_hand() -> Vec<Card> {
        vec![
            Card {
                id: "strike".to_string(),
                name: "Strike".to_string(),
                cost: 1,
                description: "Deal 6 damage".to_string(),
                effects: vec![CardEffect::Damage(6)],
                image_path: Some("assets/images/cards/strike.png".to_string()),
            },
            Card {
                id: "guard".to_string(),
                name: "Guard".to_string(),
                cost: 1,
                description: "Gain 5 Block".to_string(),
                effects: vec![CardEffect::Block(5)],
                image_path: Some("assets/images/cards/guard.png".to_string()),
            },
        ]
    }
}
