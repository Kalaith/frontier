//! Mission selection state - choose which mission to embark on

use macroquad::prelude::*;
use crate::kingdom::Roster;
use crate::missions::{Mission, load_missions};
use super::{StateTransition, MissionState};

/// State for selecting a mission before departure
pub struct MissionSelectState {
    pub missions: Vec<Mission>,
    pub selected_mission: usize,
    pub adventurer_name: String,
    pub adventurer_id: String,
    pub adventurer_hp: i32,
    pub adventurer_max_hp: i32,
    pub adventurer_stress: i32,
    pub adventurer_image: Option<String>,
}

impl MissionSelectState {
    /// Create mission select with a pre-selected adventurer
    pub fn new(adventurer_id: String, adventurer_name: String, hp: i32, max_hp: i32, stress: i32, image: Option<String>) -> Self {
        Self {
            missions: load_missions(),
            selected_mission: 0,
            adventurer_name,
            adventurer_id,
            adventurer_hp: hp,
            adventurer_max_hp: max_hp,
            adventurer_stress: stress,
            adventurer_image: image,
        }
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
                let mission_state = MissionState::from_mission_with_stats(
                    mission.clone(),
                    self.adventurer_id.clone(),
                    self.adventurer_name.clone(),
                    self.adventurer_hp,
                    self.adventurer_max_hp,
                    self.adventurer_stress,
                    self.adventurer_image.clone(),
                );
                return Some(StateTransition::ToMission(mission_state));
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
        draw_text(&format!("Adventurer: {}", self.adventurer_name), 20.0, 75.0, 20.0, SKYBLUE);
        draw_text(
            &format!("HP: {}/{}  Stress: {}", self.adventurer_hp, self.adventurer_max_hp, self.adventurer_stress),
            300.0, 75.0, 18.0, GREEN
        );
        
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
