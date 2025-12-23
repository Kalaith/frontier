//! Region definitions - areas that can be stabilized
//!
//! Regions are not conquered, only stabilized. Each has threats and unknowns.

use serde::{Deserialize, Serialize};

/// A region in the wilds
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub description: String,
    
    /// Current threat level (0-100)
    pub threat_level: i32,
    
    /// How much is known about this region (0-100)
    pub knowledge: i32,
    
    /// Is this region accessible?
    pub unlocked: bool,
    
    /// Has the first expedition been completed?
    pub discovered: bool,
    
    /// The defining horror or faction
    pub faction: String,
    
    /// Unknown traits revealed through exploration
    pub traits_revealed: Vec<String>,
    pub traits_hidden: Vec<String>,
}

impl Region {
    /// Create the starting Dark Woods region
    pub fn dark_woods() -> Self {
        Self {
            id: "dark_woods".to_string(),
            name: "The Dark Woods".to_string(),
            description: "Dense forest with twisted paths. Something moves between the trees.".to_string(),
            threat_level: 30,
            knowledge: 10,
            unlocked: true,
            discovered: false,
            faction: "The Wild Hunt".to_string(),
            traits_revealed: vec![],
            traits_hidden: vec![
                "Tangled Paths".to_string(),
                "Shadow Beasts".to_string(),
                "The Watcher".to_string(),
            ],
        }
    }
    
    /// Reveal a hidden trait
    pub fn reveal_trait(&mut self) -> Option<String> {
        if !self.traits_hidden.is_empty() {
            let revealed = self.traits_hidden.remove(0);
            self.traits_revealed.push(revealed.clone());
            self.knowledge += 10;
            Some(revealed)
        } else {
            None
        }
    }
    
    /// Stabilizing reduces threat but may have consequences
    pub fn stabilize(&mut self, amount: i32) {
        self.threat_level = (self.threat_level - amount).max(0);
    }
    
    /// When left alone, threat slowly rebuilds
    pub fn destabilize(&mut self, amount: i32) {
        self.threat_level = (self.threat_level + amount).min(100);
    }
}
