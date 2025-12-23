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
    pub cost_gold: i32,
    pub cost_supplies: i32,
}

impl Building {
    pub fn all_starter() -> Vec<Self> {
        vec![
            Self::infirmary(),
            Self::chapel(),
            Self::foundry(),
            Self::guild_hall(),
            Self::watchtowers(),
        ]
    }

    pub fn infirmary() -> Self {
        Self {
            id: "infirmary".to_string(),
            name: "Infirmary".to_string(),
            description: "Heal injuries. Unlocks 'Heal' action.".to_string(),
            built: false,
            level: 0,
            cost_gold: 50,
            cost_supplies: 20,
        }
    }
    
    pub fn chapel() -> Self {
        Self {
            id: "chapel".to_string(),
            name: "Chapel".to_string(),
            description: "Reduce stress. Unlocks 'Tavern/Prayer' action.".to_string(),
            built: false,
            level: 0,
            cost_gold: 50,
            cost_supplies: 10,
        }
    }
    
    pub fn foundry() -> Self {
        Self {
            id: "foundry".to_string(),
            name: "Foundry".to_string(),
            description: "Upgrade cards and gear via crafting.".to_string(),
            built: false,
            level: 0,
            cost_gold: 100,
            cost_supplies: 50,
        }
    }
    
    pub fn guild_hall() -> Self {
        Self {
            id: "guild_hall".to_string(),
            name: "Guild Hall".to_string(),
            description: "Recruit specialists and better adventurers.".to_string(),
            built: true, // Basic tent exists
            level: 1,
            cost_gold: 150,
            cost_supplies: 50,
        }
    }
    
    pub fn watchtowers() -> Self {
        Self {
            id: "watchtowers".to_string(),
            name: "Watchtowers".to_string(),
            description: "Safer routes, but stronger enemies attracted.".to_string(),
            built: false,
            level: 0,
            cost_gold: 80,
            cost_supplies: 40,
        }
    }
}
