//! Save and load system
//!
//! Human-readable JSON saves with version tracking.

use macroquad_toolkit::persistence::{json_key_exists, load_json_key, save_json_key};
use serde::{Deserialize, Serialize};

use crate::kingdom::{KingdomState, Roster};

/// Version for save file compatibility
const SAVE_VERSION: u32 = 1;
const SAVE_FILE_NAME: &str = "frontier_kingdom_save.json";
const GAME_NAME: &str = "frontier_kingdom";

/// Complete save data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub kingdom: KingdomState,
    pub roster: Roster,
    pub unlocked_cards: Vec<String>,
    pub unlocked_buildings: Vec<String>,
    pub regions_explored: Vec<String>,
    pub total_missions: u32,
    pub total_deaths: u32,
}

// Enable toolkit persistence methods

impl SaveData {
    /// Create a new save from current game state
    pub fn new(kingdom: KingdomState, roster: Roster) -> Self {
        Self {
            version: SAVE_VERSION,
            kingdom,
            roster,
            unlocked_cards: vec![],
            unlocked_buildings: vec![],
            regions_explored: vec![],
            total_missions: 0,
            total_deaths: 0,
        }
    }

    /// Save to a file
    pub fn save(&self, _path: &str) -> Result<(), String> {
        save_json_key(GAME_NAME, SAVE_FILE_NAME, self)
    }

    /// Load from a file
    pub fn load(_path: &str) -> Result<Self, String> {
        let save: SaveData = load_json_key(GAME_NAME, SAVE_FILE_NAME)?;

        // Version check
        if save.version > SAVE_VERSION {
            return Err(format!(
                "Save file version {} is newer than supported version {}",
                save.version, SAVE_VERSION
            ));
        }

        Ok(save)
    }

    /// Check if a save exists
    pub fn exists(_path: &str) -> bool {
        json_key_exists(GAME_NAME, SAVE_FILE_NAME)
    }

    /// Default save path (kept for compatibility with callers, though we use `get_app_data_path` internally now)
    pub fn default_path() -> String {
        "legacy_path_ignored".to_string()
    }
}

/// Create saves directory if needed
/// (Deprecated: toolkit handles directory creation)
pub fn ensure_save_directory() -> Result<(), String> {
    Ok(())
}
