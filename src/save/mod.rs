//! Save and load system
//!
//! Human-readable JSON saves with version tracking.


use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use macroquad_toolkit::persistence::{save_json, load_json, get_app_data_path, file_exists};

use crate::kingdom::{KingdomState, Roster};

/// Version for save file compatibility
const SAVE_VERSION: u32 = 1;
const SAVE_FILE_NAME: &str = "frontier_kingdom_save.json";

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
    
    fn get_save_path() -> PathBuf {
        // Try to use app data path, fallback to local file "saves/save.json" for legacy reasons or "frontier_kingdom_save.json"
        get_app_data_path("frontier_kingdom", SAVE_FILE_NAME)
            .unwrap_or_else(|| PathBuf::from(SAVE_FILE_NAME))
    }

    /// Save to a file
    pub fn save(&self, _path: &str) -> Result<(), String> {
        // We ignore the passed path argument to enforce standard location, 
        // or we could use it if we really wanted to support custom paths.
        // For now, let's stick to the standard app data path for better cross-platform support.
        save_json(Self::get_save_path(), self)
    }
    
    /// Load from a file
    pub fn load(_path: &str) -> Result<Self, String> {
        let save: SaveData = load_json(Self::get_save_path())?;
        
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
        file_exists(Self::get_save_path())
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
