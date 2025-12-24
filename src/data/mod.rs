//! Data loading - JSON content for cards, enemies, regions
//!
//! All game content is data-driven. This module handles loading from JSON files.

use std::fs;
use serde::de::DeserializeOwned;

pub mod cards;
pub mod enemies;

pub use enemies::random_enemy_for_difficulty;
// CardData and EnemyData are used internally

/// Load any JSON data file into a deserializable type
pub fn load_json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path, e))
}

/// Load JSON from the assets directory
pub fn load_asset<T: DeserializeOwned>(filename: &str) -> Result<T, String> {
    let path = format!("assets/{}", filename);
    load_json(&path)
}
