//! Mission state - expedition flow with events and encounters

use macroquad::prelude::*;
use crate::missions::Mission;
use super::{StateTransition, ResultState};
use super::combat::{CombatState, MissionContext};

/// Active mission/expedition state
pub struct MissionState {
    pub mission: Mission,
    pub current_node: usize,
    pub adventurer_id: String,
    pub adventurer_name: String,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            mission: Mission::first_mission(),
            current_node: 0,
            adventurer_id: String::new(),
            adventurer_name: "Unknown".to_string(),
        }
    }
}

impl MissionState {
    /// Create a new mission with the given adventurer (simple version)
    pub fn new(adventurer_id: String, adventurer_name: String) -> Self {
        Self {
            adventurer_id,
            adventurer_name,
            ..Default::default()
        }
    }
    
    /// Create from a Mission object with adventurer info
    pub fn from_mission(mission: Mission, adventurer_id: String, adventurer_name: String) -> Self {
        Self {
            mission,
            current_node: 0,
            adventurer_id,
            adventurer_name,
        }
    }
    
    /// Set the current node (used when returning from combat)
    pub fn with_node(mut self, node: usize) -> Self {
        self.current_node = node;
        self
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Space to advance/encounter
        if is_key_pressed(KeyCode::Space) {
            self.current_node += 1;
            
            // Mission complete check first
            if self.current_node >= self.mission.length {
                let mut results = ResultState::victory_for(&self.adventurer_id);
                results.stress_gained = self.mission.base_stress;
                results.rewards = vec![
                    format!("+{} Supplies", self.mission.reward_supplies),
                    format!("+{} Knowledge", self.mission.reward_knowledge),
                ];
                return Some(StateTransition::ToResults(results));
            }
            
            // Check for encounter at this node (odd nodes have encounters)
            if self.current_node % 2 == 1 {
                let context = MissionContext {
                    mission: self.mission.clone(),
                    current_node: self.current_node,
                    adventurer_id: self.adventurer_id.clone(),
                    adventurer_name: self.adventurer_name.clone(),
                };
                let combat = CombatState::for_mission(context);
                return Some(StateTransition::ToCombat(combat));
            }
        }
        
        // Escape to retreat
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self) {
        draw_text(&format!("MISSION: {}", self.mission.name), 20.0, 40.0, 28.0, WHITE);
        draw_text(&format!("{:?} Mission", self.mission.mission_type), 20.0, 70.0, 18.0, GRAY);
        draw_text(&format!("Adventurer: {}", self.adventurer_name), 20.0, 95.0, 20.0, SKYBLUE);
        
        // Draw progress
        let progress = format!("Node {}/{}", self.current_node + 1, self.mission.length);
        draw_text(&progress, 20.0, 130.0, 22.0, YELLOW);
        
        // Draw visual node progress
        let node_width = 80.0;
        let start_x = 20.0;
        let y = 170.0;
        
        for i in 0..self.mission.length {
            let x = start_x + (i as f32 * (node_width + 20.0));
            let color = if i < self.current_node {
                GREEN
            } else if i == self.current_node {
                YELLOW
            } else {
                DARKGRAY
            };
            draw_rectangle(x, y, node_width, 40.0, color);
            
            // Mark encounter nodes
            let is_encounter = i % 2 == 1;
            let label = if is_encounter { "!" } else { &format!("{}", i + 1) };
            draw_text(label, x + 35.0, y + 28.0, 24.0, BLACK);
        }
        
        draw_text("[SPACE] Advance  [ESC] Retreat", 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
