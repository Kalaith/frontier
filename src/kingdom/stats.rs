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
            gold: 120,
            security: 30,
            morale: 50,
            supplies: 65,
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
    /// Global danger pressure. Higher values scale mission difficulty and stress.
    #[serde(default = "default_threat_level")]
    pub threat_level: i32,
    /// Last kingdom event shown on the base screen.
    #[serde(default)]
    pub last_event: Option<String>,
    /// True once the permanent citadel ending has been achieved.
    #[serde(default)]
    pub game_won: bool,
    /// IDs of completed missions (for unlock requirements)
    #[serde(default)]
    pub completed_missions: Vec<String>,
}

fn default_threat_level() -> i32 {
    1
}

impl Default for KingdomState {
    fn default() -> Self {
        Self {
            stats: KingdomStats::new(),
            day: 1,
            buildings: crate::kingdom::Building::all_starter(),
            threat_level: default_threat_level(),
            last_event: None,
            game_won: false,
            completed_missions: vec![],
        }
    }
}

impl KingdomState {
    /// Add newly introduced buildings to old saves without disturbing existing progress.
    pub fn ensure_current_buildings(&mut self) {
        for building in crate::kingdom::Building::all_starter() {
            if !self.buildings.iter().any(|b| b.id == building.id) {
                self.buildings.push(building);
            }
        }

        if self.has_building("citadel") {
            self.game_won = true;
        }
    }

    pub fn has_building(&self, building_id: &str) -> bool {
        self.buildings
            .iter()
            .any(|b| b.id == building_id && b.built)
    }

    pub fn threat_difficulty_bonus(&self) -> i32 {
        (self.threat_level / 25).max(0)
    }

    pub fn scaled_stress_bonus(&self) -> i32 {
        self.threat_level / 30
    }

    pub fn record_mission_complete(&mut self, mission_id: &str) {
        if !self.completed_missions.iter().any(|id| id == mission_id) {
            self.completed_missions.push(mission_id.to_string());
        }
    }

    pub fn advance_threat(&mut self, victory: bool) {
        let built_count = self.buildings.iter().filter(|b| b.built).count() as i32;
        let growth = if victory { 3 } else { 8 } + (built_count / 2);
        self.threat_level = (self.threat_level + growth).clamp(1, 100);
    }

    pub fn quest_log(&self) -> Vec<(String, bool)> {
        vec![
            (
                "Scout the Dark Woods".to_string(),
                self.completed_missions
                    .iter()
                    .any(|id| id == "scout_dark_woods"),
            ),
            (
                "Build Watchtowers".to_string(),
                self.has_building("watchtowers"),
            ),
            (
                "Clear the Ruined Outpost".to_string(),
                self.completed_missions
                    .iter()
                    .any(|id| id == "clear_outpost"),
            ),
            ("Establish the Permanent Citadel".to_string(), self.game_won),
        ]
    }
}
