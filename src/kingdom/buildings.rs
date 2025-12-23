//! Buildings - unlock options, not raw power

use serde::{Deserialize, Serialize};

/// A building in the kingdom base
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub description: String,
    pub built: bool,
    pub level: i32,
}

impl Building {
    pub fn infirmary() -> Self {
        Self {
            id: "infirmary".to_string(),
            name: "Infirmary".to_string(),
            description: "Heal injuries after expeditions".to_string(),
            built: false,
            level: 0,
        }
    }
    
    pub fn chapel() -> Self {
        Self {
            id: "chapel".to_string(),
            name: "Chapel".to_string(),
            description: "Reduce stress, increase tension".to_string(),
            built: false,
            level: 0,
        }
    }
    
    pub fn foundry() -> Self {
        Self {
            id: "foundry".to_string(),
            name: "Foundry".to_string(),
            description: "Upgrade cards and gear".to_string(),
            built: false,
            level: 0,
        }
    }
}
