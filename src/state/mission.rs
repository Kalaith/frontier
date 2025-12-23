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
    pub adventurer_hp: i32,
    pub adventurer_max_hp: i32,
    pub adventurer_stress: i32,
    pub adventurer_image: Option<String>,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            mission: Mission::first_mission(),
            current_node: 0,
            adventurer_id: String::new(),
            adventurer_name: "Unknown".to_string(),
            adventurer_hp: 50,
            adventurer_max_hp: 50,
            adventurer_stress: 0,
            adventurer_image: None,
        }
    }
}

impl MissionState {
    /// Create a new mission with the given adventurer (simple version)
    pub fn new(adventurer_id: String, adventurer_name: String, adventurer_image: Option<String>) -> Self {
        Self {
            adventurer_id,
            adventurer_name,
            adventurer_image,
            ..Default::default()
        }
    }
    
    /// Create from a Mission object with adventurer info
    pub fn from_mission(mission: Mission, adventurer_id: String, adventurer_name: String, image: Option<String>) -> Self {
        Self {
            mission,
            current_node: 0,
            adventurer_id,
            adventurer_name,
            adventurer_hp: 50,
            adventurer_max_hp: 50,
            adventurer_stress: 0,
            adventurer_image: image,
        }
    }
    
    /// Create from a Mission object with full adventurer stats
    pub fn from_mission_with_stats(
        mission: Mission,
        adventurer_id: String,
        adventurer_name: String,
        hp: i32,
        max_hp: i32,
        stress: i32,
        image: Option<String>,
    ) -> Self {
        Self {
            mission,
            current_node: 0,
            adventurer_id,
            adventurer_name,
            adventurer_hp: hp,
            adventurer_max_hp: max_hp,
            adventurer_stress: stress,
            adventurer_image: image,
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
                // Pass final HP/stress to results
                results.final_hp = Some(self.adventurer_hp);
                results.final_stress = Some(self.adventurer_stress);
                return Some(StateTransition::ToResults(results));
            }
            
            // Odd nodes = combat encounters
            if self.current_node % 2 == 1 {
                let context = MissionContext {
                    mission: self.mission.clone(),
                    current_node: self.current_node,
                    adventurer_id: self.adventurer_id.clone(),
                    adventurer_name: self.adventurer_name.clone(),
                    adventurer_hp: self.adventurer_hp,
                    adventurer_max_hp: self.adventurer_max_hp,
                    adventurer_stress: self.adventurer_stress,
                    adventurer_image: self.adventurer_image.clone(),
                };
                let combat = CombatState::for_mission(context);
                return Some(StateTransition::ToCombat(combat));
            }
            
            // Even nodes (except 0) = events
            if self.current_node > 0 && self.current_node % 2 == 0 {
                if let Some(event) = crate::missions::events::random_event(self.current_node, &self.mission.region_id) {
                    let event_state = super::EventState::new(
                        event,
                        self.adventurer_id.clone(),
                        self.adventurer_name.clone(),
                    );
                    return Some(StateTransition::ToEvent(event_state));
                }
            }
        }
        
        // Escape to retreat
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self, textures: &std::collections::HashMap<String, Texture2D>) {
        // Draw background
        let bg_path = format!("assets/images/regions/{}.png", self.mission.region_id);
        if let Some(tex) = textures.get(&bg_path) {
            draw_texture_ex(
                tex,
                0.0, 0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                }
            );
            
            // Dark overlay for readability
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 150));
        }

        draw_text(&format!("MISSION: {}", self.mission.name), 20.0, 40.0, 28.0, WHITE);
        draw_text(&format!("{:?} Mission", self.mission.mission_type), 20.0, 70.0, 18.0, GRAY);
        draw_text(&format!("Adventurer: {}", self.adventurer_name), 20.0, 95.0, 20.0, SKYBLUE);
        
        // Adventurer image
        if let Some(path) = &self.adventurer_image {
            if let Some(tex) = textures.get(path) {
                draw_texture_ex(
                    tex,
                    650.0, 20.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(100.0, 100.0)),
                        ..Default::default()
                    }
                );
            }
        }
        
        // Show current adventurer status
        draw_text(
            &format!("HP: {}/{}  Stress: {}", self.adventurer_hp, self.adventurer_max_hp, self.adventurer_stress),
            300.0, 95.0, 18.0, 
            if self.adventurer_hp < self.adventurer_max_hp / 2 { RED } else { GREEN }
        );
        
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
