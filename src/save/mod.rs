//! Save and load system
//!
//! Human-readable JSON saves with version tracking.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::kingdom::{KingdomState, KingdomStats, Roster};

/// Version for save file compatibility
const SAVE_VERSION: u32 = 1;

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
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize save: {}", e))?;
        
        fs::write(path, json)
            .map_err(|e| format!("Failed to write save file: {}", e))?;
        
        Ok(())
    }
    
    /// Load from a file
    pub fn load(path: &str) -> Result<Self, String> {
        if !Path::new(path).exists() {
            return Err(format!("Save file not found: {}", path));
        }
        
        let json = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read save file: {}", e))?;
        
        let save: SaveData = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse save file: {}", e))?;
        
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
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// Default save path
    pub fn default_path() -> String {
        "saves/save.json".to_string()
    }
}

/// Create saves directory if needed
pub fn ensure_save_directory() -> Result<(), String> {
    let dir = Path::new("saves");
    if !dir.exists() {
        fs::create_dir_all(dir)
            .map_err(|e| format!("Failed to create saves directory: {}", e))?;
    }
    Ok(())
}
