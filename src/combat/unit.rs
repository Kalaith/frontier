//! Combat units - players and enemies

use serde::{Deserialize, Serialize};

/// What an enemy intends to do next turn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnemyIntent {
    Attack(i32),      // Damage amount
    Block(i32),       // Block amount
    Buff,             // Strengthening self
    Debuff,           // Weakening player
    Unknown,          // Intent not yet revealed
}

impl EnemyIntent {
    /// Get display text for the intent
    pub fn description(&self) -> String {
        match self {
            EnemyIntent::Attack(dmg) => format!("Attack {}", dmg),
            EnemyIntent::Block(amt) => format!("Block {}", amt),
            EnemyIntent::Buff => "Buff".to_string(),
            EnemyIntent::Debuff => "Debuff".to_string(),
            EnemyIntent::Unknown => "???".to_string(),
        }
    }
}

/// A combat unit (player adventurer or enemy)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub block: i32,
    pub stress: i32,
    pub is_player: bool,
    pub image_path: Option<String>,
    pub base_damage: i32,
    pub intent: EnemyIntent,
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
            image_path: None,
            base_damage: 0,
            intent: EnemyIntent::Unknown,
        }
    }
    
    pub fn new_enemy(name: &str, max_hp: i32, image_path: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            hp: max_hp,
            max_hp,
            block: 0,
            stress: 0,
            is_player: false,
            image_path,
            base_damage: 6,
            intent: EnemyIntent::Attack(6),
        }
    }
    
    /// Create enemy with specific base damage
    pub fn new_enemy_with_damage(name: &str, max_hp: i32, base_damage: i32, image_path: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            hp: max_hp,
            max_hp,
            block: 0,
            stress: 0,
            is_player: false,
            image_path,
            base_damage,
            intent: EnemyIntent::Attack(base_damage),
        }
    }
    
    /// Roll a new intent based on enemy AI pattern
    pub fn roll_intent(&mut self, turn: usize) {
        if self.is_player { return; }
        
        // Simple pattern: Attack most turns, occasionally block
        let pattern = turn % 4;
        self.intent = match pattern {
            0 => EnemyIntent::Attack(self.base_damage),
            1 => EnemyIntent::Attack(self.base_damage + 2),
            2 => EnemyIntent::Block(5),
            3 => EnemyIntent::Attack(self.base_damage),
            _ => EnemyIntent::Attack(self.base_damage),
        };
    }
    
    /// Execute the current intent, returning damage dealt (if attack)
    pub fn execute_intent(&mut self) -> (i32, i32) {
        match self.intent {
            EnemyIntent::Attack(dmg) => (dmg, 0),
            EnemyIntent::Block(amt) => {
                self.block += amt;
                (0, 0)
            }
            EnemyIntent::Buff => {
                self.base_damage += 2;
                (0, 0)
            }
            EnemyIntent::Debuff => (0, 2), // Returns stress to add
            EnemyIntent::Unknown => (0, 0),
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
