//! Roster management - the adventurer pool

use serde::{Deserialize, Serialize};
use super::adventurer::{Adventurer, AdventurerClass};

/// The kingdom's adventurer roster
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Roster {
    pub adventurers: Vec<Adventurer>,
    pub graveyard: Vec<Adventurer>,  // Fallen heroes
}

impl Roster {
    pub fn new() -> Self {
        Self {
            adventurers: vec![],
            graveyard: vec![],
        }
    }
    
    /// Create a starting roster with basic adventurers
    pub fn starter() -> Self {
        let mut roster = Self::new();
        
        // The initial unideal roster (per GDD)
        roster.adventurers.push(Adventurer::new("Marcus", AdventurerClass::Soldier));
        roster.adventurers.push(Adventurer::new("Elena", AdventurerClass::Scout));
        roster.adventurers.push(Adventurer::new("Brother Aldric", AdventurerClass::Healer));
        
        // Give them some initial stress - they know the danger
        for adv in &mut roster.adventurers {
            adv.stress = 10;
        }
        
        roster
    }
    
    /// Get all available adventurers (not injured, not too stressed)
    pub fn available(&self) -> Vec<&Adventurer> {
        self.adventurers
            .iter()
            .filter(|a| a.available && !a.is_stressed())
            .collect()
    }
    
    /// Get adventurer by ID
    pub fn get(&self, id: &str) -> Option<&Adventurer> {
        self.adventurers.iter().find(|a| a.id == id)
    }
    
    /// Get mutable adventurer by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Adventurer> {
        self.adventurers.iter_mut().find(|a| a.id == id)
    }
    
    /// Record a death
    pub fn record_death(&mut self, id: &str) {
        if let Some(pos) = self.adventurers.iter().position(|a| a.id == id) {
            let fallen = self.adventurers.remove(pos);
            self.graveyard.push(fallen);
        }
    }
    
    /// Add a new adventurer (recruited, hired, etc.)
    pub fn add(&mut self, adventurer: Adventurer) {
        self.adventurers.push(adventurer);
    }
    
    /// Count living adventurers
    pub fn count(&self) -> usize {
        self.adventurers.len()
    }
    
    /// Count fallen adventurers
    pub fn fallen_count(&self) -> usize {
        self.graveyard.len()
    }
}
