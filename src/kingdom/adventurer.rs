//! Adventurer - persistent characters that remember

use serde::{Deserialize, Serialize};
use crate::combat::Card;

/// An adventurer in the kingdom's roster
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Adventurer {
    pub id: String,
    pub name: String,
    pub class: AdventurerClass,
    
    // Stats
    pub hp: i32,
    pub max_hp: i32,
    pub stress: i32,
    pub level: i32,
    pub xp: i32,
    
    // Persistent conditions
    pub traits: Vec<Trait>,
    pub injuries: Vec<Injury>,
    pub traumas: Vec<Trauma>,
    
    // Cards this adventurer has unlocked/added
    pub deck_additions: Vec<String>, // Card IDs
    
    // State
    pub available: bool,
    pub missions_completed: u32,
    pub kills: u32,
}

impl Adventurer {
    pub fn new(name: &str, class: AdventurerClass) -> Self {
        let base_hp = match class {
            AdventurerClass::Soldier => 45,
            AdventurerClass::Scout => 35,
            AdventurerClass::Healer => 30,
            AdventurerClass::Mystic => 32,
        };
        
        Self {
            id: uuid_simple(),
            name: name.to_string(),
            class,
            hp: base_hp,
            max_hp: base_hp,
            stress: 0,
            level: 1,
            xp: 0,
            traits: vec![],
            injuries: vec![],
            traumas: vec![],
            deck_additions: vec![],
            available: true,
            missions_completed: 0,
            kills: 0,
        }
    }
    
    /// Check if adventurer is too stressed to deploy
    pub fn is_stressed(&self) -> bool {
        self.stress >= 50
    }
    
    /// Check if adventurer is injured
    pub fn is_injured(&self) -> bool {
        !self.injuries.is_empty()
    }
    
    /// Apply stress, potentially triggering trauma
    pub fn add_stress(&mut self, amount: i32) -> Option<Trauma> {
        self.stress += amount;
        
        // Trauma check at thresholds
        if self.stress >= 100 && !self.has_trauma(TraumaType::Broken) {
            let trauma = Trauma::new(TraumaType::Broken);
            self.traumas.push(trauma.clone());
            return Some(trauma);
        } else if self.stress >= 75 && !self.has_trauma(TraumaType::Paranoid) {
            let trauma = Trauma::new(TraumaType::Paranoid);
            self.traumas.push(trauma.clone());
            return Some(trauma);
        } else if self.stress >= 50 && !self.has_trauma(TraumaType::Fearful) {
            let trauma = Trauma::new(TraumaType::Fearful);
            self.traumas.push(trauma.clone());
            return Some(trauma);
        }
        
        None
    }
    
    fn has_trauma(&self, trauma_type: TraumaType) -> bool {
        self.traumas.iter().any(|t| t.trauma_type == trauma_type)
    }
    
    /// Reduce stress (at base, costs resources)
    pub fn reduce_stress(&mut self, amount: i32) {
        self.stress = (self.stress - amount).max(0);
    }
    
    /// Heal HP (at infirmary)
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

/// Simple UUID generator (timestamp-based for uniqueness)
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("adv_{}", now.as_nanos())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AdventurerClass {
    Soldier,  // High HP, attack cards
    Scout,    // Low HP, utility/evasion
    Healer,   // Low HP, stress/healing
    Mystic,   // Medium HP, special effects
}

/// Positive or negative traits affecting gameplay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trait {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_positive: bool,
}

/// Physical injuries from combat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Injury {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: i32,  // 1-3
    pub healing_days: i32,
}

impl Injury {
    pub fn wounded_leg() -> Self {
        Self {
            id: "wounded_leg".to_string(),
            name: "Wounded Leg".to_string(),
            description: "Movement cards cost +1 energy".to_string(),
            severity: 2,
            healing_days: 3,
        }
    }
    
    pub fn broken_arm() -> Self {
        Self {
            id: "broken_arm".to_string(),
            name: "Broken Arm".to_string(),
            description: "Attack cards deal -2 damage".to_string(),
            severity: 3,
            healing_days: 5,
        }
    }
}

/// Psychological trauma from stress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trauma {
    pub trauma_type: TraumaType,
    pub severity: i32,
}

impl Trauma {
    pub fn new(trauma_type: TraumaType) -> Self {
        Self {
            trauma_type,
            severity: 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TraumaType {
    Fearful,   // Chance to skip turn
    Paranoid,  // Block cards cost +1
    Broken,    // All cards cost +1
    Hopeless,  // Cannot reduce stress in combat
}
