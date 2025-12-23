//! Kingdom stats - the core tension system

use serde::{Deserialize, Serialize};

/// Core kingdom stats that pull against each other
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KingdomStats {
    /// Currency for buildings and upgrades
    pub gold: i32,
    /// Road safety, encounter predictability
    pub security: i32,
    /// Willingness to send people out
    pub morale: i32,
    /// Equipment and expedition readiness
    pub supplies: i32,
    /// Understanding enemies and regions
    pub knowledge: i32,
    /// How other factions respond
    pub influence: i32,
}

impl KingdomStats {
    pub fn new() -> Self {
        Self {
            gold: 100,
            security: 30,
            morale: 50,
            supplies: 50,
            knowledge: 10,
            influence: 20,
        }
    }
}

/// Full kingdom state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KingdomState {
    pub stats: KingdomStats,
    pub day: u32,
    pub buildings: Vec<crate::kingdom::Building>,
}

impl Default for KingdomState {
    fn default() -> Self {
        Self {
            stats: KingdomStats::new(),
            day: 1,
            buildings: crate::kingdom::Building::all_starter(),
        }
    }
}
