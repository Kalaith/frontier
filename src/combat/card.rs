//! Card definitions and starter deck

use serde::{Deserialize, Serialize};
use super::CardEffect;

/// Card class restriction - which adventurer classes can use this card
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CardClass {
    #[default]
    Any,
    Soldier,
    Scout,
    Healer,
    Mystic,
}

impl CardClass {
    /// Check if a class string matches this CardClass
    pub fn matches(&self, class_name: &str) -> bool {
        match self {
            CardClass::Any => true,
            CardClass::Soldier => class_name == "Soldier",
            CardClass::Scout => class_name == "Scout",
            CardClass::Healer => class_name == "Healer",
            CardClass::Mystic => class_name == "Mystic",
        }
    }
}

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
    #[serde(default)]
    pub class: CardClass,
}

impl Card {
    /// Check if this card is an attack (deals damage)
    pub fn is_attack(&self) -> bool {
        self.effects.iter().any(|e| matches!(e, 
            CardEffect::Damage(_) | 
            CardEffect::DamageIfNoBlock { .. } | 
            CardEffect::DamageIfLowHp { .. } |
            CardEffect::DamageIfEnemyActed { .. } |
            CardEffect::DamageIfVulnerable { .. }
        ))
    }
    
    /// Check if this card can be used by the given class
    #[allow(dead_code)]
    pub fn usable_by(&self, class_name: &str) -> bool {
        self.class.matches(class_name)
    }
    
    /// Load cards for a specific class (includes "Any" class cards)
    pub fn load_for_class(class_name: &str) -> Vec<Card> {
        match crate::data::cards::CardData::load_all() {
            Ok(all_cards) => {
                all_cards.iter()
                    .filter(|c| c.class_matches(class_name))
                    .map(|c| c.to_card())
                    .collect()
            }
            Err(e) => {
                eprintln!("Failed to load cards from JSON: {}. Using fallback.", e);
                Self::fallback_starter_hand()
            }
        }
    }
    
    /// Load starter hand for a class
    pub fn starter_hand_for_class(class_name: &str) -> Vec<Card> {
        let cards = Self::load_for_class(class_name);
        cards.into_iter().take(5).collect()
    }
    
    /// Load the starter hand from JSON data (legacy - uses Any cards)
    pub fn starter_hand() -> Vec<Card> {
        Self::load_starter_deck()
            .into_iter()
            .take(5)
            .collect()
    }
    
    /// Load full starter deck from JSON (legacy)
    #[allow(dead_code)]
    pub fn load_starter_deck() -> Vec<Card> {
        match crate::data::cards::CardData::load_all() {
            Ok(all_cards) => {
                // Just get basic cards for legacy support
                all_cards.iter()
                    .filter(|c| c.class_matches("Any"))
                    .map(|c| c.to_card())
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
                class: CardClass::Any,
            },
            Card {
                id: "guard".to_string(),
                name: "Guard".to_string(),
                cost: 1,
                description: "Gain 5 Block".to_string(),
                effects: vec![CardEffect::Block(5)],
                image_path: Some("assets/images/cards/guard.png".to_string()),
                class: CardClass::Any,
            },
        ]
    }
}

