//! Combat units - players and enemies

use serde::{Deserialize, Serialize};

/// A combat unit (player adventurer or enemy)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub block: i32,
    pub stress: i32,
    pub is_player: bool,
}

impl Unit {
    pub fn new_player(name: &str, max_hp: i32) -> Self {
        Self {
            name: name.to_string(),
            hp: max_hp,
            max_hp,
            block: 0,
            stress: 0,
            is_player: true,
        }
    }
    
    pub fn new_enemy(name: &str, max_hp: i32) -> Self {
        Self {
            name: name.to_string(),
            hp: max_hp,
            max_hp,
            block: 0,
            stress: 0,
            is_player: false,
        }
    }
    
    pub fn take_damage(&mut self, amount: i32) {
        let blocked = amount.min(self.block);
        self.block -= blocked;
        let remaining = amount - blocked;
        self.hp -= remaining;
    }
    
    pub fn add_block(&mut self, amount: i32) {
        self.block += amount;
    }
    
    pub fn add_stress(&mut self, amount: i32) {
        self.stress += amount;
    }
    
    pub fn reduce_stress(&mut self, amount: i32) {
        self.stress = (self.stress - amount).max(0);
    }
}
