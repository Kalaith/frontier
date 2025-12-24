//! Party management - groups of adventurers that go on missions together

use serde::{Deserialize, Serialize};
use super::adventurer::Adventurer;

/// Maximum party size
pub const MAX_PARTY_SIZE: usize = 4;

/// A party of adventurers ready to embark on a mission
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    /// IDs of adventurers in the party (first is leader)
    pub member_ids: Vec<String>,
}

impl Party {
    pub fn new() -> Self {
        Self {
            member_ids: Vec::new(),
        }
    }
    
    /// Create a party with a single leader
    pub fn with_leader(leader_id: &str) -> Self {
        Self {
            member_ids: vec![leader_id.to_string()],
        }
    }
    
    /// Get the leader's ID (first member)
    pub fn leader_id(&self) -> Option<&str> {
        self.member_ids.first().map(|s| s.as_str())
    }
    
    /// Check if party is full
    pub fn is_full(&self) -> bool {
        self.member_ids.len() >= MAX_PARTY_SIZE
    }
    
    /// Check if party is empty
    pub fn is_empty(&self) -> bool {
        self.member_ids.is_empty()
    }
    
    /// Number of members
    pub fn size(&self) -> usize {
        self.member_ids.len()
    }
    
    /// Add a member to the party
    pub fn add_member(&mut self, id: &str) -> bool {
        if self.is_full() || self.member_ids.contains(&id.to_string()) {
            return false;
        }
        self.member_ids.push(id.to_string());
        true
    }
    
    /// Remove a member from the party
    pub fn remove_member(&mut self, id: &str) -> bool {
        if let Some(pos) = self.member_ids.iter().position(|m| m == id) {
            self.member_ids.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Check if an adventurer is in the party
    pub fn contains(&self, id: &str) -> bool {
        self.member_ids.iter().any(|m| m == id)
    }
}

impl Default for Party {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of a party member's state for use in missions/combat
#[derive(Clone, Debug)]
pub struct PartyMemberState {
    pub id: String,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub stress: i32,
    pub image_path: Option<String>,
    pub class_name: String,
}

impl PartyMemberState {
    /// Create from an adventurer
    pub fn from_adventurer(adv: &Adventurer) -> Self {
        Self {
            id: adv.id.clone(),
            name: adv.name.clone(),
            hp: adv.hp,
            max_hp: adv.max_hp,
            stress: adv.stress,
            image_path: adv.image_path.clone(),
            class_name: format!("{:?}", adv.class),
        }
    }
}
