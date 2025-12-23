//! Kingdom base state - manage adventurers, buildings, prepare expeditions

use macroquad::prelude::*;
use crate::kingdom::{KingdomState, Roster};
use super::{StateTransition, MissionSelectState};

/// Focus area for input
#[derive(PartialEq)]
pub enum FocusArea {
    Roster,
    Buildings,
}

impl Default for FocusArea {
    fn default() -> Self {
        FocusArea::Roster
    }
}

/// State for managing the kingdom base
#[derive(Default)]
pub struct BaseState {
    pub selected_building: Option<usize>,
    pub selected_adventurer: Option<usize>,
    pub focus: FocusArea,
}

impl BaseState {
    pub fn update(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) -> Option<StateTransition> {
        // Toggle focus
        if is_key_pressed(KeyCode::Tab) {
            self.focus = match self.focus {
                FocusArea::Roster => FocusArea::Buildings,
                FocusArea::Buildings => FocusArea::Roster,
            };
            // Clear selections to avoid confusion? Or keep them?
            // Keeping them allows quick switch back.
        }
        
        match self.focus {
            FocusArea::Roster => {
                // Adventurer selection
                let available_count = roster.adventurers.len().min(9);
                for i in 0..available_count {
                    let key = match i {
                        0 => KeyCode::Key1, 1 => KeyCode::Key2, 2 => KeyCode::Key3,
                        3 => KeyCode::Key4, 4 => KeyCode::Key5, 5 => KeyCode::Key6,
                        6 => KeyCode::Key7, 7 => KeyCode::Key8, 8 => KeyCode::Key9,
                        _ => continue,
                    };
                    if is_key_pressed(key) {
                        self.selected_adventurer = Some(i);
                        self.selected_building = None; // Deselect building
                    }
                }
                
                // Roster Actions
                if let Some(adv_idx) = self.selected_adventurer {
                    if adv_idx < roster.adventurers.len() {
                        // M: Mission
                        if is_key_pressed(KeyCode::M) {
                            let adventurer = &roster.adventurers[adv_idx];
                            let select = MissionSelectState::new(
                                adventurer.id.clone(),
                                adventurer.name.clone(),
                                adventurer.hp,
                                adventurer.max_hp,
                                adventurer.stress,
                                adventurer.image_path.clone(),
                            );
                            return Some(StateTransition::ToMissionSelect(select));
                        }
                        
                        // H: Heal (Unlocks with Infirmary)
                        if is_key_pressed(KeyCode::H) {
                            let has_infirmary = kingdom.buildings.iter().any(|b| b.id == "infirmary" && b.built);
                            if has_infirmary && kingdom.stats.supplies >= 10 {
                                if let Some(adv) = roster.adventurers.get_mut(adv_idx) {
                                    if adv.hp < adv.max_hp {
                                        adv.heal(10);
                                        kingdom.stats.supplies -= 10;
                                    }
                                }
                            }
                        }
                        
                        // T: Tavern (Unlocks with Chapel)
                        if is_key_pressed(KeyCode::T) {
                            let has_chapel = kingdom.buildings.iter().any(|b| b.id == "chapel" && b.built);
                            if has_chapel && kingdom.stats.supplies >= 10 {
                                if let Some(adv) = roster.adventurers.get_mut(adv_idx) {
                                    if adv.stress > 0 {
                                        adv.reduce_stress(20);
                                        kingdom.stats.supplies -= 10;
                                    }
                                }
                            }
                        }
                    }
                }
            },
            FocusArea::Buildings => {
                // Building selection
                let count = kingdom.buildings.len().min(9);
                for i in 0..count {
                    let key = match i {
                        0 => KeyCode::Key1, 1 => KeyCode::Key2, 2 => KeyCode::Key3,
                        3 => KeyCode::Key4, 4 => KeyCode::Key5, 5 => KeyCode::Key6,
                        6 => KeyCode::Key7, 7 => KeyCode::Key8, 8 => KeyCode::Key9,
                        _ => continue,
                    };
                    if is_key_pressed(key) {
                        self.selected_building = Some(i);
                        self.selected_adventurer = None; // Deselect adventurer
                    }
                }
                
                // Construction
                if is_key_pressed(KeyCode::Enter) {
                    if let Some(idx) = self.selected_building {
                        if let Some(building) = kingdom.buildings.get_mut(idx) {
                            if !building.built && 
                               kingdom.stats.gold >= building.cost_gold && 
                               kingdom.stats.supplies >= building.cost_supplies 
                            {
                                kingdom.stats.gold -= building.cost_gold;
                                kingdom.stats.supplies -= building.cost_supplies;
                                building.built = true;
                                building.level = 1;
                            }
                        }
                    }
                }
            }
        }
        
        // Global Actions
        // R to recruit (Unlocks with Guild Hall)
        if is_key_pressed(KeyCode::R) {
            let has_guild = kingdom.buildings.iter().any(|b| b.id == "guild_hall" && b.built);
            if has_guild {
                return Some(StateTransition::ToRecruit);
            }
        }
        
        None
    }
    
    pub fn draw(&self, kingdom: &KingdomState, roster: &Roster, textures: &std::collections::HashMap<String, Texture2D>) {
        // Draw title
        draw_text("FRONTIER KINGDOM", 20.0, 40.0, 32.0, WHITE);
        draw_text("Manage your kingdom. [TAB] Switch Focus", 20.0, 70.0, 18.0, GRAY);
        
        // Draw kingdom stats
        let stats = &kingdom.stats;
        let y_start = 120.0;
        draw_text(&format!("Gold: {}", stats.gold), 20.0, y_start, 20.0, YELLOW);
        draw_text(&format!("Supplies: {}", stats.supplies), 20.0, y_start + 25.0, 20.0, YELLOW);
        draw_text(&format!("Security: {}", stats.security), 20.0, y_start + 50.0, 20.0, SKYBLUE);
        draw_text(&format!("Morale: {}", stats.morale), 20.0, y_start + 75.0, 20.0, ORANGE);
        draw_text(&format!("Knowledge: {}", stats.knowledge), 20.0, y_start + 100.0, 20.0, PURPLE);
        
        // Draw selected adventurer image large if selected (and in roster focus)
        if self.focus == FocusArea::Roster {
            if let Some(idx) = self.selected_adventurer {
                if let Some(adv) = roster.adventurers.get(idx) {
                    if let Some(path) = &adv.image_path {
                        if let Some(tex) = textures.get(path) {
                            draw_texture_ex(
                                tex,
                                20.0, y_start + 150.0,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(vec2(250.0, 250.0)),
                                    ..Default::default()
                                }
                            );
                        }
                    }
                }
            }
        }
        
        // --- ROSTER PANEL ---
        let roster_x = 300.0;
        let roster_y = 120.0;
        let roster_color = if self.focus == FocusArea::Roster { YELLOW } else { WHITE };
        draw_text("ADVENTURERS", roster_x, roster_y, 24.0, roster_color);
        
        for (i, adv) in roster.adventurers.iter().enumerate() {
            let y = roster_y + 35.0 + (i as f32 * 70.0);
            let is_selected = self.selected_adventurer == Some(i) && self.focus == FocusArea::Roster;
            
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
        
        // --- BUILDINGS PANEL ---
        let build_x = 700.0;
        let build_y = 120.0;
        let build_color = if self.focus == FocusArea::Buildings { YELLOW } else { WHITE };
        draw_text("BUILDINGS", build_x, build_y, 24.0, build_color);
        
        for (i, b) in kingdom.buildings.iter().enumerate() {
            let y = build_y + 35.0 + (i as f32 * 70.0);
            let is_selected = self.selected_building == Some(i) && self.focus == FocusArea::Buildings;
            
            // Background
            let bg_color = if b.built {
                Color::from_rgba(60, 60, 80, 255)
            } else {
                Color::from_rgba(30, 30, 30, 255)
            };
            
            draw_rectangle(build_x, y - 15.0, 300.0, 60.0, bg_color);
            if is_selected {
                draw_rectangle_lines(build_x, y - 15.0, 300.0, 60.0, 2.0, YELLOW);
            }
            
            // Text
            let name_color = if b.built { WHITE } else { GRAY };
            draw_text(&format!("[{}] {}", i + 1, b.name), build_x + 10.0, y + 5.0, 20.0, name_color);
            
            if b.built {
                draw_text("Constructed", build_x + 10.0, y + 25.0, 16.0, GREEN);
            } else {
                draw_text(&format!("Cost: {}g, {}s", b.cost_gold, b.cost_supplies), build_x + 10.0, y + 25.0, 16.0, RED);
            }
        }
        
        // Draw instructions
        let instruction = match self.focus {
            FocusArea::Roster => {
                if self.selected_adventurer.is_some() {
                    let mut s = String::from("[M] Launch Mission");
                    let has_infirmary = kingdom.buildings.iter().any(|b| b.id == "infirmary" && b.built);
                    let has_chapel = kingdom.buildings.iter().any(|b| b.id == "chapel" && b.built);
                    let has_guild = kingdom.buildings.iter().any(|b| b.id == "guild_hall" && b.built);
                    
                    if has_infirmary { s.push_str("  [H] Heal(10s)"); }
                    if has_chapel { s.push_str("  [T] Tavern(10s)"); }
                    if has_guild { s.push_str("  [R] Recruit"); }
                    
                    s
                } else {
                    "[1-9] Select Adventurer".to_string()
                }
            },
            FocusArea::Buildings => {
                 if let Some(idx) = self.selected_building {
                     if let Some(b) = kingdom.buildings.get(idx) {
                         if !b.built {
                             "[ENTER] Construct Building".to_string()
                         } else {
                            "Building Active".to_string()
                         }
                     } else { "".to_string() }
                 } else {
                     "[1-9] Select Building".to_string()
                 }
            }
        };
        
        draw_text(&instruction, 20.0, screen_height() - 40.0, 20.0, GREEN);
        draw_text("[F5] Save  [F9] Load  [TAB] Switch Focus", 20.0, screen_height() - 20.0, 16.0, GRAY);
    }
}
