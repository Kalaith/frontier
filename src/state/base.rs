//! Kingdom base state - manage adventurers, buildings, prepare expeditions

use macroquad::prelude::*;
use crate::kingdom::{KingdomState, Roster};
use super::{StateTransition, MissionSelectState};

/// State for managing the kingdom base
#[derive(Default)]
pub struct BaseState {
    pub selected_building: Option<usize>,
    pub selected_adventurer: Option<usize>,
}

impl BaseState {
    pub fn update(&mut self, _kingdom: &mut KingdomState, roster: &Roster) -> Option<StateTransition> {
        // Adventurer selection with number keys
        let available_count = roster.adventurers.len().min(9);
        for i in 0..available_count {
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
                self.selected_adventurer = Some(i);
            }
        }
        
        // Check for mission launch - requires adventurer selection
        if is_key_pressed(KeyCode::M) {
            if let Some(adv_idx) = self.selected_adventurer {
                if adv_idx < roster.adventurers.len() {
                    let adventurer = &roster.adventurers[adv_idx];
                    let select = MissionSelectState::new(
                        adventurer.id.clone(),
                        adventurer.name.clone(),
                    );
                    return Some(StateTransition::ToMissionSelect(select));
                }
            }
        }
        
        None
    }
    
    pub fn draw(&self, kingdom: &KingdomState, roster: &Roster) {
        // Draw title
        draw_text("FRONTIER KINGDOM", 20.0, 40.0, 32.0, WHITE);
        draw_text("The kingdom is unfinished. Survival is costly.", 20.0, 70.0, 18.0, GRAY);
        
        // Draw kingdom stats
        let stats = &kingdom.stats;
        let y_start = 120.0;
        draw_text(&format!("Security: {}", stats.security), 20.0, y_start, 20.0, YELLOW);
        draw_text(&format!("Morale: {}", stats.morale), 20.0, y_start + 25.0, 20.0, YELLOW);
        draw_text(&format!("Supplies: {}", stats.supplies), 20.0, y_start + 50.0, 20.0, YELLOW);
        draw_text(&format!("Knowledge: {}", stats.knowledge), 20.0, y_start + 75.0, 20.0, YELLOW);
        draw_text(&format!("Influence: {}", stats.influence), 20.0, y_start + 100.0, 20.0, YELLOW);
        
        // Draw adventurer roster
        let roster_x = 300.0;
        let roster_y = 120.0;
        draw_text("ADVENTURERS", roster_x, roster_y, 24.0, WHITE);
        
        for (i, adv) in roster.adventurers.iter().enumerate() {
            let y = roster_y + 35.0 + (i as f32 * 70.0);
            let is_selected = self.selected_adventurer == Some(i);
            
            // Card background
            let bg_color = if is_selected {
                Color::from_rgba(60, 80, 60, 255)
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(roster_x, y - 15.0, 350.0, 60.0, bg_color);
            
            if is_selected {
                draw_rectangle_lines(roster_x, y - 15.0, 350.0, 60.0, 2.0, GREEN);
            }
            
            // Adventurer info
            let name_color = if is_selected { GREEN } else { WHITE };
            draw_text(&format!("[{}] {}", i + 1, adv.name), roster_x + 10.0, y + 5.0, 20.0, name_color);
            draw_text(&format!("{:?}", adv.class), roster_x + 200.0, y + 5.0, 16.0, GRAY);
            
            // Stats bar
            let hp_pct = adv.hp as f32 / adv.max_hp as f32;
            let stress_pct = adv.stress as f32 / 100.0;
            
            // HP bar
            draw_rectangle(roster_x + 10.0, y + 15.0, 100.0, 8.0, DARKGRAY);
            draw_rectangle(roster_x + 10.0, y + 15.0, 100.0 * hp_pct, 8.0, GREEN);
            draw_text(&format!("{}/{}", adv.hp, adv.max_hp), roster_x + 115.0, y + 23.0, 14.0, GREEN);
            
            // Stress bar
            draw_rectangle(roster_x + 170.0, y + 15.0, 100.0, 8.0, DARKGRAY);
            draw_rectangle(roster_x + 170.0, y + 15.0, 100.0 * stress_pct, 8.0, ORANGE);
            draw_text(&format!("Stress: {}", adv.stress), roster_x + 275.0, y + 23.0, 14.0, ORANGE);
        }
        
        // Draw instructions
        let instruction = if self.selected_adventurer.is_some() {
            "[M] Launch Mission  [1-3] Select Adventurer"
        } else {
            "[1-3] Select Adventurer to Launch Mission"
        };
        draw_text(instruction, 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
