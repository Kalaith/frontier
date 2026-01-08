//! Enemy data loading from JSON

use serde::{Deserialize, Serialize};
use crate::combat::Unit;

/// Enemy template from data file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub id: String,
    pub name: String,
    pub max_hp: i32,
    pub base_damage: i32,
    pub threat_level: i32,
    pub region: String,
    #[serde(default)]
    pub image_path: Option<String>,
}

impl EnemyData {
    /// Load all enemies from the enemies.json asset file
    pub fn load_all() -> Result<Vec<EnemyData>, String> {
        crate::load_asset!("enemies.json", Vec<EnemyData>)
    }
    
    /// Convert to a combat Unit
    pub fn to_unit(&self) -> Unit {
        Unit::new_enemy_with_damage(&self.name, self.max_hp, self.base_damage, self.image_path.clone())
    }
}

/// Get a random enemy appropriate for the given difficulty
pub fn random_enemy_for_difficulty(difficulty: i32) -> Unit {

    
    match EnemyData::load_all() {
        Ok(enemies) => {
            // Filter to enemies at or below the difficulty
            let suitable: Vec<_> = enemies.iter()
                .filter(|e| e.threat_level <= difficulty)
                .collect();
            
            if suitable.is_empty() {
                // Fallback to any enemy
                if let Some(enemy) = enemies.first() {
                    return enemy.to_unit();
                }
            } else {
                // Pick a random one
                if let Some(enemy) = macroquad_toolkit::rng::choose(&suitable) {
                    return enemy.to_unit();
                }
            }
            
            // Ultimate fallback
            Unit::new_enemy("Forest Beast", 30, None)
        }
        Err(_) => Unit::new_enemy("Forest Beast", 30, None)
    }
}

