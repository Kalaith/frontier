//! Card definitions and starter deck

use serde::{Deserialize, Serialize};
use super::CardEffect;

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
    /// Generate the starter hand of cards
    pub fn starter_hand() -> Vec<Card> {
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
            Card {
                id: "desperate_swing".to_string(),
                name: "Desperate Swing".to_string(),
                cost: 0,
                description: "Deal 5 dmg, +5 Stress".to_string(),
                effects: vec![CardEffect::Damage(5), CardEffect::SelfStress(5)],
                image_path: Some("assets/images/cards/desperate_swing.png".to_string()),
            },
            Card {
                id: "recenter".to_string(),
                name: "Recenter".to_string(),
                cost: 1,
                description: "Reduce Stress by 6".to_string(),
                effects: vec![CardEffect::ReduceStress(6)],
                image_path: Some("assets/images/cards/recenter.png".to_string()),
            },
            Card {
                id: "measured_strike".to_string(),
                name: "Measured Strike".to_string(),
                cost: 2,
                description: "Deal 10 damage".to_string(),
                effects: vec![CardEffect::Damage(10)],
                image_path: Some("assets/images/cards/measured_strike.png".to_string()),
            },
        ]
    }
}
