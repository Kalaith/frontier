//! Combat resolution - effects are validated and applied here

use super::{CardEffect, Unit};

/// Resolves card effects into state changes
pub struct CombatResolver {
    /// Log of resolved effects for replay/debugging
    pub log: Vec<String>,
}

impl CombatResolver {
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }
    
    /// Resolve an effect from player to target (or self)
    pub fn resolve(&mut self, effect: &CardEffect, player: &mut Unit, target: &mut Unit) {
        match effect {
            CardEffect::Damage(amount) => {
                target.take_damage(*amount);
                self.log.push(format!("{} takes {} damage", target.name, amount));
            }
            CardEffect::Block(amount) => {
                player.add_block(*amount);
                self.log.push(format!("{} gains {} block", player.name, amount));
            }
            CardEffect::Stress(amount) => {
                target.add_stress(*amount);
                self.log.push(format!("{} gains {} stress", target.name, amount));
            }
            CardEffect::SelfStress(amount) => {
                player.add_stress(*amount);
                self.log.push(format!("{} gains {} stress (self)", player.name, amount));
            }
            CardEffect::ReduceStress(amount) => {
                player.reduce_stress(*amount);
                self.log.push(format!("{} reduces stress by {}", player.name, amount));
            }
            CardEffect::DamageIfNoBlock { base, bonus } => {
                let total = if target.block == 0 { base + bonus } else { *base };
                target.take_damage(total);
                self.log.push(format!("{} takes {} damage", target.name, total));
            }
            CardEffect::DamageIfLowHp { base, bonus, threshold_percent } => {
                let threshold = (player.max_hp * threshold_percent) / 100;
                let total = if player.hp < threshold { base + bonus } else { *base };
                target.take_damage(total);
                self.log.push(format!("{} takes {} damage", target.name, total));
            }
        }
    }
}

impl Default for CombatResolver {
    fn default() -> Self {
        Self::new()
    }
}
