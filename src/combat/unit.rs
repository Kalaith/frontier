//! Combat units - players and enemies

use serde::{Deserialize, Serialize};
use crate::kingdom::{StatusEffect, StatusType};

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
    pub statuses: Vec<StatusEffect>,
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
            statuses: vec![],
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
            statuses: vec![],
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
            statuses: vec![],
        }
    }
    
    /// Roll a new intent based on enemy AI pattern
    pub fn roll_intent(&mut self, turn: usize) {
        if self.is_player { return; }
        
        // Check for Stun status
        if self.has_status(StatusType::Stun) {
            self.intent = EnemyIntent::Unknown; // Or explicit Stun intent
            return;
        }
        
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
        if self.has_status(StatusType::Stun) {
            return (0, 0);
        }
        
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
    
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let mut final_damage = amount;
        
        // Vulnerable: +50% damage
        if self.has_status(StatusType::Vulnerable) {
            final_damage = (final_damage as f32 * 1.5) as i32;
        }
        
        let blocked = final_damage.min(self.block);
        self.block -= blocked;
        let remaining = final_damage - blocked;
        self.hp -= remaining;
        
        remaining // Return actual damage taken
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
    
    pub fn add_status(&mut self, effect: StatusEffect) {
        // Check if existing status of same type, extend duration or value?
        // Simple simplified rule: Add to list. (Duplicates stack intensity?)
        // Or unique by type?
        // Let's replace if exists for simplicity, or add if unique.
        if let Some(existing) = self.statuses.iter_mut().find(|s| s.effect_type == effect.effect_type) {
             existing.duration = existing.duration.max(effect.duration);
             if effect.value > existing.value {
                 existing.value = effect.value;
             }
        } else {
            self.statuses.push(effect);
        }
    }
    
    pub fn has_status(&self, status_type: StatusType) -> bool {
        self.statuses.iter().any(|s| s.effect_type == status_type)
    }
    
    pub fn tick_statuses(&mut self) {
        // Apply end-of-turn effects like Regen/Bleed here?
        // Or simplify: just reduce duration.
        // Let's implement Regen here.
        
        let mut hp_change = 0;
        
        self.statuses.retain_mut(|s| {
            if s.effect_type == StatusType::Regen {
                hp_change += s.value;
            }
            
            s.duration -= 1;
            s.duration > 0
        });
        
        if hp_change > 0 {
            self.hp = (self.hp + hp_change).min(self.max_hp);
        }
    }
}
