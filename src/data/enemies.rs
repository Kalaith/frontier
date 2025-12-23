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
}

impl EnemyData {
    /// Load all enemies from the enemies.json asset file
    pub fn load_all() -> Result<Vec<EnemyData>, String> {
        super::load_asset("enemies.json")
    }
    
    /// Convert to a combat Unit
    pub fn to_unit(&self) -> Unit {
        Unit::new_enemy(&self.name, self.max_hp)
    }
}
