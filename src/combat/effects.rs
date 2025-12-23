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
    /// Conditional damage if target has no block
    DamageIfNoBlock { base: i32, bonus: i32 },
    /// Conditional damage if HP below percentage
    DamageIfLowHp { base: i32, bonus: i32, threshold_percent: i32 },
}
