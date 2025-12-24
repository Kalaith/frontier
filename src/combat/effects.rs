//! Card effects - what cards emit for resolution

use serde::{Deserialize, Serialize};

/// Effects emitted by cards - resolved by CombatResolver
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CardEffect {
    /// Deal damage to target
    Damage(i32),
    /// Gain block on self
    Block(i32),
    /// Add stress to target
    Stress(i32),
    /// Add stress to self (cost)
    SelfStress(i32),
    /// Reduce own stress
    ReduceStress(i32),
    /// Heal self HP
    Heal(i32),
    /// Draw additional cards
    DrawCards(i32),
    /// Gain energy this turn
    GainEnergy(i32),
    /// Gain energy next turn
    GainEnergyNextTurn(i32),
    /// Clear all debuffs from self
    ClearDebuffs,
    /// Apply stress to enemy
    EnemyStress(i32),
    /// Conditional damage if target has no block
    DamageIfNoBlock { base: i32, bonus: i32 },
    /// Conditional damage if HP below percentage
    DamageIfLowHp { base: i32, bonus: i32, threshold_percent: i32 },
    /// Conditional damage if enemy acted last turn
    DamageIfEnemyActed { base: i32, bonus: i32 },
    /// Conditional damage if target is Vulnerable
    DamageIfVulnerable { base: i32, bonus: i32 },
    /// Apply a status effect
    ApplyStatus {
        effect_type: crate::kingdom::StatusType,
        duration: i32,
        value: i32,
        target_self: bool, 
    },
    /// Reduce incoming stress by percentage for this turn (e.g., 50 = 50%)
    StressResistance(i32),
    /// Disable playing attack cards for the rest of this turn
    DisableAttacks,
}
