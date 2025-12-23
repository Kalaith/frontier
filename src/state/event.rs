//! Event state - narrative encounters with choices

use macroquad::prelude::*;
use std::collections::HashMap;
use crate::missions::events::{Event, EventOutcome};
use super::StateTransition;

/// State for handling mission events
pub struct EventState {
    pub event: Event,
    pub selected_choice: usize,
    pub adventurer_id: String,
    pub adventurer_name: String,
    pub return_to_mission: bool,
    /// Applied outcomes to pass back
    pub stress_change: i32,
    pub hp_change: i32,
    pub supplies_change: i32,
    pub knowledge_change: i32,
    pub trigger_combat: Option<String>,
    pub skip_node: bool,
}

impl EventState {
    pub fn new(event: Event, adventurer_id: String, adventurer_name: String) -> Self {
        Self {
            event,
            selected_choice: 0,
            adventurer_id,
            adventurer_name,
            return_to_mission: false,
            stress_change: 0,
            hp_change: 0,
            supplies_change: 0,
            knowledge_change: 0,
            trigger_combat: None,
            skip_node: false,
        }
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Choice selection
        let choice_count = self.event.choices.len();
        
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_choice > 0 {
                self.selected_choice -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_choice < choice_count.saturating_sub(1) {
                self.selected_choice += 1;
            }
        }
        
        // Number keys
        for i in 0..choice_count.min(9) {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                _ => continue,
            };
            if is_key_pressed(key) {
                self.selected_choice = i;
            }
        }
        
        // Confirm choice
        if is_key_pressed(KeyCode::Enter) {
            if let Some(choice) = self.event.choices.get(self.selected_choice) {
                // Process outcomes
                for outcome in &choice.outcomes {
                    match outcome {
                        EventOutcome::Stress(amt) => self.stress_change += amt,
                        EventOutcome::Heal(amt) => self.hp_change += amt,
                        EventOutcome::Supplies(amt) => self.supplies_change += amt,
                        EventOutcome::Knowledge(amt) => self.knowledge_change += amt,
                        EventOutcome::Combat(enemy_id) => {
                            self.trigger_combat = Some(enemy_id.clone());
                        }
                        EventOutcome::SkipNode => self.skip_node = true,
                        EventOutcome::RevealTrait => {
                            self.knowledge_change += 5;
                        }
                        EventOutcome::Nothing => {}
                    }
                }
                self.return_to_mission = true;
                
                // For now, return to base after event
                // TODO: Return to mission state with outcomes applied
                return Some(StateTransition::ToBase);
            }
        }
        
        None
    }
    
    pub fn draw(&self, _textures: &HashMap<String, Texture2D>) {
        // Darken background
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 180));
        
        // Event panel
        let panel_x = 100.0;
        let panel_y = 80.0;
        let panel_w = screen_width() - 200.0;
        let panel_h = screen_height() - 160.0;
        
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(30, 30, 40, 250));
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, WHITE);
        
        // Title
        draw_text(&self.event.title, panel_x + 20.0, panel_y + 40.0, 32.0, YELLOW);
        
        // Description
        let desc_y = panel_y + 80.0;
        // Simple word wrap
        let max_width = panel_w - 40.0;
        let words: Vec<&str> = self.event.description.split_whitespace().collect();
        let mut line = String::new();
        let mut y = desc_y;
        
        for word in words {
            let test_line = if line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", line, word)
            };
            
            // Rough estimate: 8 pixels per character at size 18
            if test_line.len() as f32 * 8.0 > max_width {
                draw_text(&line, panel_x + 20.0, y, 18.0, LIGHTGRAY);
                y += 25.0;
                line = word.to_string();
            } else {
                line = test_line;
            }
        }
        if !line.is_empty() {
            draw_text(&line, panel_x + 20.0, y, 18.0, LIGHTGRAY);
        }
        
        // Choices
        let choices_y = panel_y + 200.0;
        draw_text("CHOOSE:", panel_x + 20.0, choices_y, 20.0, WHITE);
        
        for (i, choice) in self.event.choices.iter().enumerate() {
            let y = choices_y + 35.0 + (i as f32 * 50.0);
            let is_selected = i == self.selected_choice;
            
            // Choice background
            let bg_color = if is_selected {
                Color::from_rgba(60, 80, 60, 255)
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(panel_x + 20.0, y - 15.0, panel_w - 40.0, 45.0, bg_color);
            
            if is_selected {
                draw_rectangle_lines(panel_x + 20.0, y - 15.0, panel_w - 40.0, 45.0, 2.0, GREEN);
            }
            
            // Choice text
            let text_color = if is_selected { WHITE } else { GRAY };
            draw_text(&format!("[{}] {}", i + 1, choice.text), panel_x + 30.0, y + 10.0, 18.0, text_color);
        }
        
        // Instructions
        draw_text("[↑/↓] Select  [ENTER] Confirm", panel_x + 20.0, panel_y + panel_h - 30.0, 18.0, GREEN);
    }
}
