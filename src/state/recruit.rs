//! Recruitment state - hire new adventurers

use macroquad::prelude::*;
use std::collections::HashMap;
use crate::kingdom::{Adventurer, AdventurerClass, KingdomState, Roster};
use super::StateTransition;

/// Names for random adventurers
const FIRST_NAMES: &[&str] = &[
    "Aldric", "Beatrix", "Cedric", "Diana", "Edmund", "Freya", 
    "Godric", "Helena", "Ivan", "Jocelyn", "Klaus", "Lydia",
    "Magnus", "Nadia", "Oscar", "Petra", "Quinn", "Rosa",
    "Stefan", "Thea", "Ulric", "Vera", "Werner", "Xena",
];

/// A recruit available for hire
#[derive(Clone)]
pub struct Recruit {
    pub adventurer: Adventurer,
    pub cost: i32,
}

impl Recruit {
    pub fn random(class: AdventurerClass) -> Self {
        let name = FIRST_NAMES[rand::gen_range(0, FIRST_NAMES.len())];
        let gender = if rand::gen_range(0, 2) == 0 { 
            crate::kingdom::Gender::Male 
        } else { 
            crate::kingdom::Gender::Female 
        };
        
        let cost = match class {
            AdventurerClass::Soldier => 50,
            AdventurerClass::Scout => 40,
            AdventurerClass::Healer => 60,
            AdventurerClass::Mystic => 70,
        };
        let adventurer = Adventurer::new(name, class, gender);
        Self { adventurer, cost }
    }
}

/// State for recruiting new adventurers
pub struct RecruitState {
    pub recruits: Vec<Recruit>,
    pub selected: usize,
}

impl Default for RecruitState {
    fn default() -> Self {
        Self::new()
    }
}

impl RecruitState {
    pub fn new() -> Self {
        // Generate 3 random recruits
        let recruits = vec![
            Recruit::random(AdventurerClass::Soldier),
            Recruit::random(AdventurerClass::Scout),
            Recruit::random(AdventurerClass::Healer),
        ];
        
        Self {
            recruits,
            selected: 0,
        }
    }
    
    pub fn update(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) -> Option<StateTransition> {
        // Selection
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected > 0 {
                self.selected -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected < self.recruits.len().saturating_sub(1) {
                self.selected += 1;
            }
        }
        
        // Number keys
        for i in 0..self.recruits.len().min(9) {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                _ => continue,
            };
            if is_key_pressed(key) {
                self.selected = i;
            }
        }
        
        // Hire with Enter
        if is_key_pressed(KeyCode::Enter) {
            if let Some(recruit) = self.recruits.get(self.selected) {
                if kingdom.stats.gold >= recruit.cost {
                    kingdom.stats.gold -= recruit.cost;
                    roster.add(recruit.adventurer.clone());
                    self.recruits.remove(self.selected);
                    if self.selected >= self.recruits.len() && self.selected > 0 {
                        self.selected -= 1;
                    }
                }
            }
        }
        
        // Escape to return
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self, kingdom: &KingdomState, textures: &HashMap<String, Texture2D>) {
        draw_text("RECRUITMENT", 20.0, 40.0, 32.0, WHITE);
        draw_text(&format!("Gold: {}", kingdom.stats.gold), 20.0, 70.0, 20.0, YELLOW);
        
        let start_y = 120.0;
        let card_height = 120.0;
        let card_width = 500.0;
        
        for (i, recruit) in self.recruits.iter().enumerate() {
            let y = start_y + (i as f32 * (card_height + 10.0));
            let is_selected = i == self.selected;
            let can_afford = kingdom.stats.gold >= recruit.cost;
            // ... (rest of helper) ...
            
            // Background
            let bg_color = if is_selected {
                if can_afford {
                    Color::from_rgba(60, 80, 60, 255)
                } else {
                    Color::from_rgba(80, 60, 60, 255)
                }
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(20.0, y, card_width, card_height, bg_color);
            
            if is_selected {
                let border = if can_afford { GREEN } else { RED };
                draw_rectangle_lines(20.0, y, card_width, card_height, 2.0, border);
            }
            
            // Portrait
            if let Some(path) = &recruit.adventurer.image_path {
                if let Some(tex) = textures.get(path) {
                    draw_texture_ex(
                        tex, 30.0, y + 10.0, WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(100.0, 100.0)),
                            ..Default::default()
                        }
                    );
                }
            }
            
            // Info
            let text_color = if is_selected { WHITE } else { GRAY };
            draw_text(&format!("[{}] {}", i + 1, recruit.adventurer.name), 140.0, y + 30.0, 22.0, text_color);
            draw_text(&format!("{:?} - {:?}", recruit.adventurer.gender, recruit.adventurer.class), 140.0, y + 55.0, 18.0, SKYBLUE);
            draw_text(&format!("HP: {}", recruit.adventurer.max_hp), 140.0, y + 80.0, 16.0, GREEN);
            
            // Cost
            let cost_color = if can_afford { YELLOW } else { RED };
            draw_text(&format!("Cost: {} Gold", recruit.cost), 350.0, y + 55.0, 18.0, cost_color);
        }
        
        if self.recruits.is_empty() {
            draw_text("No recruits available", 20.0, start_y + 30.0, 24.0, GRAY);
        }
        
        draw_text("[↑/↓] Select  [ENTER] Hire  [ESC] Back", 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
