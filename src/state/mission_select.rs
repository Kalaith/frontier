//! Mission selection state - choose which mission to embark on

use macroquad::prelude::*;
use crate::kingdom::{Roster, Party, PartyMemberState};
use crate::missions::{Mission, load_missions};
use super::{StateTransition, MissionState};

/// State for selecting a mission before departure
pub struct MissionSelectState {
    pub missions: Vec<Mission>,
    pub selected_mission: usize,
    /// Party members going on this mission (leader is first)
    pub party_members: Vec<PartyMemberState>,
}

impl MissionSelectState {
    /// Create mission select with a pre-selected adventurer (backwards compatible, single-person party)
    #[allow(dead_code)]
    pub fn new(adventurer_id: String, adventurer_name: String, hp: i32, max_hp: i32, stress: i32, image: Option<String>) -> Self {
        let member = PartyMemberState {
            id: adventurer_id,
            name: adventurer_name,
            hp,
            max_hp,
            stress,
            image_path: image,
        };
        Self {
            missions: load_missions(),
            selected_mission: 0,
            party_members: vec![member],
        }
    }
    
    /// Create mission select from a party and roster
    pub fn for_party(party: Party, roster: &Roster) -> Self {
        let party_members: Vec<PartyMemberState> = party.member_ids
            .iter()
            .filter_map(|id| roster.get(id))
            .map(|adv| PartyMemberState::from_adventurer(adv))
            .collect();
        
        Self {
            missions: load_missions(),
            selected_mission: 0,
            party_members,
        }
    }
    
    /// Get the party leader's info
    pub fn leader(&self) -> Option<&PartyMemberState> {
        self.party_members.first()
    }
    
    pub fn update(&mut self, _roster: &Roster) -> Option<StateTransition> {
        // Mission selection with arrow keys or number keys
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_mission > 0 {
                self.selected_mission -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_mission < self.missions.len().saturating_sub(1) {
                self.selected_mission += 1;
            }
        }
        
        // Number keys for quick selection
        for i in 0..self.missions.len().min(9) {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                5 => KeyCode::Key6,
                6 => KeyCode::Key7,
                7 => KeyCode::Key8,
                8 => KeyCode::Key9,
                _ => continue,
            };
            
            if is_key_pressed(key) {
                self.selected_mission = i;
            }
        }
        
        // Confirm mission with Enter
        if is_key_pressed(KeyCode::Enter) {
            if let Some(mission) = self.missions.get(self.selected_mission) {
                // Use leader's stats for primary mission state
                if let Some(_leader) = self.leader() {
                    let mission_state = MissionState::from_mission_with_party(
                        mission.clone(),
                        self.party_members.clone(),
                    );
                    return Some(StateTransition::ToMission(mission_state));
                }
            }
        }
        
        // Cancel with Escape
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self, textures: &std::collections::HashMap<String, Texture2D>) {
        draw_text("SELECT MISSION", 20.0, 40.0, 32.0, WHITE);
        
        // Show party members
        let party_label = if self.party_members.len() == 1 {
            format!("Adventurer: {}", self.party_members.first().map(|m| m.name.as_str()).unwrap_or("?"))
        } else {
            format!("Party ({} members):", self.party_members.len())
        };
        draw_text(&party_label, 20.0, 75.0, 20.0, SKYBLUE);
        
        // Draw party member portraits and info
        let portrait_size = 60.0;
        let portrait_gap = 10.0;
        let portrait_start_x = 200.0;
        
        for (i, member) in self.party_members.iter().enumerate() {
            let x = portrait_start_x + (i as f32 * (portrait_size + portrait_gap + 80.0));
            
            // Portrait
            if let Some(path) = &member.image_path {
                if let Some(tex) = textures.get(path) {
                    draw_texture_ex(
                        tex,
                        x, 55.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(portrait_size, portrait_size)),
                            ..Default::default()
                        }
                    );
                }
            }
            
            // Leader indicator
            if i == 0 {
                draw_text("★", x + portrait_size - 15.0, 68.0, 16.0, YELLOW);
            }
            
            // Name and HP below portrait
            draw_text(&member.name, x + portrait_size + 5.0, 75.0, 14.0, WHITE);
            draw_text(
                &format!("HP:{}/{}", member.hp, member.max_hp),
                x + portrait_size + 5.0, 90.0, 12.0, GREEN
            );
            draw_text(
                &format!("Str:{}", member.stress),
                x + portrait_size + 5.0, 104.0, 12.0, ORANGE
            );
        }
        
        let start_y = 120.0;
        let card_height = 100.0;
        let card_width = 600.0;
        
        for (i, mission) in self.missions.iter().enumerate() {
            let y = start_y + (i as f32 * (card_height + 10.0));
            let is_selected = i == self.selected_mission;
            
            // Card background
            let bg_color = if is_selected {
                Color::from_rgba(60, 80, 60, 255)
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(20.0, y, card_width, card_height, bg_color);
            
            if is_selected {
                draw_rectangle_lines(20.0, y, card_width, card_height, 2.0, GREEN);
            }
            
            // Mission info
            let text_color = if is_selected { WHITE } else { GRAY };
            draw_text(&format!("[{}] {}", i + 1, mission.name), 30.0, y + 25.0, 22.0, text_color);
            draw_text(&mission.description, 30.0, y + 50.0, 16.0, GRAY);
            
            // Stats
            let type_color = match mission.mission_type {
                crate::missions::MissionType::Scout => SKYBLUE,
                crate::missions::MissionType::Suppress => RED,
                crate::missions::MissionType::Secure => GREEN,
                crate::missions::MissionType::Investigate => PURPLE,
            };
            draw_text(&format!("{:?}", mission.mission_type), 30.0, y + 75.0, 14.0, type_color);
            
            // Rewards
            draw_text(
                &format!("Rewards: +{} Supplies, +{} Knowledge", mission.reward_supplies, mission.reward_knowledge),
                200.0, y + 75.0, 14.0, YELLOW,
            );
            
            // Difficulty & Stress
            draw_text(
                &format!("Difficulty: {}  |  Stress: {}", mission.difficulty, mission.base_stress),
                450.0, y + 25.0, 14.0, ORANGE,
            );
        }
        
        // Instructions
        draw_text("[↑/↓] Select  [ENTER] Embark  [ESC] Cancel", 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
