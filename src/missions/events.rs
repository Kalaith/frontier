//! Mission events - narrative encounters and choices
//!
//! Events introduce stress before combat and teach that choices matter.

use serde::{Deserialize, Serialize};

/// An event encountered during a mission
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub choices: Vec<EventChoice>,
}

/// A choice within an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventChoice {
    pub text: String,
    pub outcomes: Vec<EventOutcome>,
}

/// Possible outcomes of a choice
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventOutcome {
    /// Add stress to party
    Stress(i32),
    /// Heal HP
    Heal(i32),
    /// Gain supplies
    Supplies(i32),
    /// Gain knowledge about region
    Knowledge(i32),
    /// Trigger a combat encounter
    Combat(String),  // Enemy ID
    /// Reveal a hidden region trait
    RevealTrait,
    /// Skip the next node
    SkipNode,
    /// Nothing happens
    Nothing,
}

impl Event {
    /// First route event - The Twisted Path
    pub fn twisted_path() -> Self {
        Self {
            id: "twisted_path".to_string(),
            title: "The Twisted Path".to_string(),
            description: "The trail splits. One path is overgrown but direct. The other is clear but winds deeper into the forest.".to_string(),
            choices: vec![
                EventChoice {
                    text: "Take the overgrown path (+5 stress, skip node)".to_string(),
                    outcomes: vec![EventOutcome::Stress(5), EventOutcome::SkipNode],
                },
                EventChoice {
                    text: "Take the clear path (possible encounter)".to_string(),
                    outcomes: vec![EventOutcome::Combat("shadow_wolf".to_string())],
                },
                EventChoice {
                    text: "Scout both carefully (+10 stress, gain knowledge)".to_string(),
                    outcomes: vec![EventOutcome::Stress(10), EventOutcome::Knowledge(5)],
                },
            ],
        }
    }
    
    /// Discovery event - Ancient Marker
    pub fn ancient_marker() -> Self {
        Self {
            id: "ancient_marker".to_string(),
            title: "Ancient Marker".to_string(),
            description: "A weathered stone marker stands at the crossroads. Strange symbols cover its surface.".to_string(),
            choices: vec![
                EventChoice {
                    text: "Study the marker (+5 stress, reveal trait)".to_string(),
                    outcomes: vec![EventOutcome::Stress(5), EventOutcome::RevealTrait],
                },
                EventChoice {
                    text: "Ignore it and continue".to_string(),
                    outcomes: vec![EventOutcome::Nothing],
                },
            ],
        }
    }
    
    /// Rest event
    pub fn forest_shrine() -> Self {
        Self {
            id: "forest_shrine".to_string(),
            title: "Forest Shrine".to_string(),
            description: "A small shrine to forgotten gods. The air feels calmer here.".to_string(),
            choices: vec![
                EventChoice {
                    text: "Rest briefly (-10 stress)".to_string(),
                    outcomes: vec![EventOutcome::Stress(-10)],
                },
                EventChoice {
                    text: "Search for offerings (+10 supplies, +5 stress)".to_string(),
                    outcomes: vec![EventOutcome::Supplies(10), EventOutcome::Stress(5)],
                },
            ],
        }
    }
}

/// Get a random event for a mission node
pub fn random_event(node: usize, _region_id: &str) -> Option<Event> {
    match node {
        0 => Some(Event::twisted_path()),
        1 => None,  // Combat node
        2 => Some(Event::ancient_marker()),
        3 => None,  // Combat node
        4 => Some(Event::forest_shrine()),
        _ => None,
    }
}
