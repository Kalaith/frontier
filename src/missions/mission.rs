//! Mission definitions and types
//!
//! Expeditions are contracts with uncertainty.

use serde::{Deserialize, Serialize};
use crate::data::load_asset;

/// Mission types from the GDD
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MissionType {
    /// Low danger, knowledge-focused
    Scout,
    /// Combat-heavy, stabilizes regions
    Suppress,
    /// Enables trade or settlement
    Secure,
    /// Narrative events, high stress
    Investigate,
}

/// A mission available to undertake
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub description: String,
    pub mission_type: MissionType,
    pub region_id: String,
    
    /// Number of nodes in the expedition
    pub length: usize,
    
    /// Base difficulty (affects encounter strength)
    pub difficulty: i32,
    
    /// Expected rewards
    pub reward_supplies: i32,
    pub reward_knowledge: i32,
    pub reward_influence: i32,
    
    /// Stress this mission is likely to cause
    pub base_stress: i32,
}

impl Mission {
    /// First available mission - Scout the Dark Woods
    pub fn first_mission() -> Self {
        Self {
            id: "scout_dark_woods".to_string(),
            name: "Scout the Dark Woods".to_string(),
            description: "Map the forest edge. Avoid direct confrontation if possible.".to_string(),
            mission_type: MissionType::Scout,
            region_id: "dark_woods".to_string(),
            length: 5,
            difficulty: 1,
            reward_supplies: 10,
            reward_knowledge: 15,
            reward_influence: 0,
            base_stress: 8,
        }
    }
    
    /// Combat-focused mission
    pub fn suppress_beasts() -> Self {
        Self {
            id: "suppress_beasts".to_string(),
            name: "Suppress Forest Beasts".to_string(),
            description: "Clear the creatures blocking the main road.".to_string(),
            mission_type: MissionType::Suppress,
            region_id: "dark_woods".to_string(),
            length: 6,
            difficulty: 2,
            reward_supplies: 20,
            reward_knowledge: 5,
            reward_influence: 10,
            base_stress: 15,
        }
    }
}

/// Load missions from the JSON asset file
pub fn load_missions() -> Vec<Mission> {
    match load_asset::<Vec<Mission>>("missions.json") {
        Ok(missions) => missions,
        Err(e) => {
            eprintln!("Warning: Could not load missions.json: {}", e);
            // Fallback to hardcoded missions
            available_missions()
        }
    }
}

/// Available missions for the player to choose from (hardcoded fallback)
pub fn available_missions() -> Vec<Mission> {
    vec![
        Mission::first_mission(),
        Mission::suppress_beasts(),
    ]
}

