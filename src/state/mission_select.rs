//! Mission selection state - choose which mission to embark on

use macroquad::prelude::*;
use crate::kingdom::{Roster, Party, PartyMemberState, KingdomState};
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
            class_name: "Soldier".to_string(),
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
    
    /// Check if a mission is unlocked
    pub fn is_mission_unlocked(&self, mission: &Mission, kingdom: &KingdomState) -> bool {
        mission.unlock_requirement.is_met(kingdom)
    }
    
    pub fn update(&mut self, _roster: &Roster, kingdom: &KingdomState) -> Option<StateTransition> {
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
        
        // Confirm mission with Enter (only if unlocked)
        if is_key_pressed(KeyCode::Enter) {
            if let Some(mission) = self.missions.get(self.selected_mission) {
                // Check if mission is unlocked
                if self.is_mission_unlocked(mission, kingdom) {
                    if let Some(_leader) = self.leader() {
                        let mission_state = MissionState::from_mission_with_party(
                            mission.clone(),
                            self.party_members.clone(),
                        );
                        return Some(StateTransition::ToMission(mission_state));
                    }
                }
            }
        }
        
        // Cancel with Escape
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self, kingdom: &KingdomState, textures: &std::collections::HashMap<String, Texture2D>) {
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
            let is_unlocked = self.is_mission_unlocked(mission, kingdom);
            
            // Card background - different for locked missions
            let bg_color = if !is_unlocked {
                Color::from_rgba(30, 30, 35, 255)  // Darker for locked
            } else if is_selected {
                Color::from_rgba(60, 80, 60, 255)
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(20.0, y, card_width, card_height, bg_color);
            
            // Border
            if is_selected {
                let border_color = if is_unlocked { GREEN } else { RED };
                draw_rectangle_lines(20.0, y, card_width, card_height, 2.0, border_color);
            }
            
            // Lock icon for locked missions
            if !is_unlocked {
                draw_text("🔒", card_width - 30.0, y + 30.0, 24.0, RED);
            }
            
            // Mission info - dimmed for locked
            let text_color = if !is_unlocked {
                Color::from_rgba(100, 100, 100, 255)
            } else if is_selected { 
                WHITE 
            } else { 
                GRAY 
            };
            draw_text(&format!("[{}] {}", i + 1, mission.name), 30.0, y + 25.0, 22.0, text_color);
            
            // Description or unlock requirement
            if !is_unlocked {
                let req_text = mission.unlock_requirement.description();
                draw_text(&req_text, 30.0, y + 50.0, 16.0, RED);
            } else {
                draw_text(&mission.description, 30.0, y + 50.0, 16.0, GRAY);
            }
            
            // Stats (dimmed for locked)
            let type_color = if !is_unlocked {
                Color::from_rgba(80, 80, 80, 255)
            } else {
                match mission.mission_type {
                    crate::missions::MissionType::Scout => SKYBLUE,
                    crate::missions::MissionType::Suppress => RED,
                    crate::missions::MissionType::Secure => GREEN,
                    crate::missions::MissionType::Investigate => PURPLE,
                }
            };
            draw_text(&format!("{:?}", mission.mission_type), 30.0, y + 75.0, 14.0, type_color);
            
            // Rewards (dimmed for locked)
            let reward_color = if is_unlocked { YELLOW } else { Color::from_rgba(100, 100, 50, 255) };
            draw_text(
                &format!("Rewards: +{} Supplies, +{} Knowledge", mission.reward_supplies, mission.reward_knowledge),
                200.0, y + 75.0, 14.0, reward_color,
            );
            
            // Difficulty & Stress
            let info_color = if is_unlocked { ORANGE } else { Color::from_rgba(100, 80, 50, 255) };
            draw_text(
                &format!("Difficulty: {}  |  Stress: {}", mission.difficulty, mission.base_stress),
                450.0, y + 25.0, 14.0, info_color,
            );
        }
        
        // Instructions
        let selected_locked = self.missions.get(self.selected_mission)
            .map(|m| !self.is_mission_unlocked(m, kingdom))
            .unwrap_or(false);
        
        let instructions = if selected_locked {
            "[↑/↓] Select  [LOCKED - Cannot Embark]  [ESC] Cancel"
        } else {
            "[↑/↓] Select  [ENTER] Embark  [ESC] Cancel"
        };
        draw_text(instructions, 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
