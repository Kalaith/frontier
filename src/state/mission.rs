//! Mission state - expedition flow with events and encounters

use macroquad::prelude::*;
use crate::missions::Mission;
use crate::kingdom::PartyMemberState;
use super::{StateTransition, ResultState};
use super::combat::{CombatState, MissionContext};

/// Active mission/expedition state
pub struct MissionState {
    pub mission: Mission,
    pub current_node: usize,
    /// All party members on this mission (first is leader)
    pub party_members: Vec<PartyMemberState>,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            mission: Mission::first_mission(),
            current_node: 0,
            party_members: vec![],
        }
    }
}

impl MissionState {
    /// Get the party leader
    pub fn leader(&self) -> Option<&PartyMemberState> {
        self.party_members.first()
    }
    
    /// Get mutable party leader
    #[allow(dead_code)]
    pub fn leader_mut(&mut self) -> Option<&mut PartyMemberState> {
        self.party_members.first_mut()
    }
    
    /// Create a new mission with the given adventurer (simple version, backwards compat)
    #[allow(dead_code)]
    pub fn new(adventurer_id: String, adventurer_name: String, adventurer_image: Option<String>) -> Self {
        let member = PartyMemberState {
            id: adventurer_id,
            name: adventurer_name,
            hp: 50,
            max_hp: 50,
            stress: 0,
            image_path: adventurer_image,
        };
        Self {
            party_members: vec![member],
            ..Default::default()
        }
    }
    
    /// Create from a Mission object with adventurer info (backwards compat)
    #[allow(dead_code)]
    pub fn from_mission(mission: Mission, adventurer_id: String, adventurer_name: String, image: Option<String>) -> Self {
        let member = PartyMemberState {
            id: adventurer_id,
            name: adventurer_name,
            hp: 50,
            max_hp: 50,
            stress: 0,
            image_path: image,
        };
        Self {
            mission,
            current_node: 0,
            party_members: vec![member],
        }
    }
    
    /// Create from a Mission object with full adventurer stats (backwards compat)
    #[allow(dead_code)]
    pub fn from_mission_with_stats(
        mission: Mission,
        adventurer_id: String,
        adventurer_name: String,
        hp: i32,
        max_hp: i32,
        stress: i32,
        image: Option<String>,
    ) -> Self {
        let member = PartyMemberState {
            id: adventurer_id,
            name: adventurer_name,
            hp,
            max_hp,
            stress,
            image_path: image,
        };
        Self {
            mission,
            current_node: 0,
            party_members: vec![member],
        }
    }
    
    /// Create from a Mission object with a full party
    pub fn from_mission_with_party(mission: Mission, party_members: Vec<PartyMemberState>) -> Self {
        Self {
            mission,
            current_node: 0,
            party_members,
        }
    }
    
    /// Set the current node (used when returning from combat)
    pub fn with_node(mut self, node: usize) -> Self {
        self.current_node = node;
        self
    }
    
    /// Update party member state after combat
    #[allow(dead_code)]
    pub fn update_member(&mut self, id: &str, hp: i32, stress: i32) {
        if let Some(member) = self.party_members.iter_mut().find(|m| m.id == id) {
            member.hp = hp;
            member.stress = stress;
        }
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Space to advance/encounter
        if is_key_pressed(KeyCode::Space) {
            self.current_node += 1;
            
            // Mission complete check first
            if self.current_node >= self.mission.length {
                if let Some(_leader) = self.leader() {
                    let mut results = ResultState::victory_for_party(&self.party_members);
                    results.stress_gained = self.mission.base_stress;
                    results.rewards = vec![
                        format!("+{} Supplies", self.mission.reward_supplies),
                        format!("+{} Knowledge", self.mission.reward_knowledge),
                    ];
                    return Some(StateTransition::ToResults(results));
                }
            }
            
            // Odd nodes = combat encounters
            if self.current_node % 2 == 1 {
                // Create combat with the full party
                let context = MissionContext {
                    mission: self.mission.clone(),
                    current_node: self.current_node,
                    party_members: self.party_members.clone(),
                };
                let combat = CombatState::for_mission(context);
                return Some(StateTransition::ToCombat(combat));
            }
            
            // Even nodes (except 0) = events
            if self.current_node > 0 && self.current_node % 2 == 0 {
                if let Some(event) = crate::missions::events::random_event(self.current_node, &self.mission.region_id) {
                    if let Some(leader) = self.leader() {
                        let event_state = super::EventState::new(
                            event,
                            leader.id.clone(),
                            leader.name.clone(),
                        ).with_mission_context(
                            self.mission.clone(),
                            self.current_node,
                            self.party_members.clone(),
                        );
                        return Some(StateTransition::ToEvent(event_state));
                    }
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
        
        // Show party members
        let party_label = if self.party_members.len() == 1 {
            format!("Adventurer: {}", self.leader().map(|m| m.name.as_str()).unwrap_or("?"))
        } else {
            format!("Party ({}):", self.party_members.len())
        };
        draw_text(&party_label, 20.0, 95.0, 20.0, SKYBLUE);
        
        // Draw party member portraits in a row
        let portrait_size = 60.0;
        let portrait_start_x = 650.0;
        
        for (i, member) in self.party_members.iter().enumerate() {
            let x = portrait_start_x + (i as f32 * (portrait_size + 5.0));
            
            // Portrait
            if let Some(path) = &member.image_path {
                if let Some(tex) = textures.get(path) {
                    // Dim portrait if HP is low
                    let tint = if member.hp < member.max_hp / 3 { 
                        Color::from_rgba(255, 100, 100, 255) 
                    } else { 
                        WHITE 
                    };
                    draw_texture_ex(
                        tex,
                        x, 30.0,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(vec2(portrait_size, portrait_size)),
                            ..Default::default()
                        }
                    );
                }
            }
            
            // HP bar under portrait
            let hp_pct = member.hp as f32 / member.max_hp as f32;
            draw_rectangle(x, 95.0, portrait_size, 6.0, DARKGRAY);
            draw_rectangle(x, 95.0, portrait_size * hp_pct, 6.0, GREEN);
            
            // Leader star
            if i == 0 {
                draw_text("★", x + portrait_size - 12.0, 42.0, 14.0, YELLOW);
            }
        }
        
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
