//! Kingdom base state - manage adventurers, buildings, prepare expeditions

use macroquad::prelude::*;
use crate::kingdom::{KingdomState, Roster, Party};
use super::{StateTransition, MissionSelectState};

/// Focus area for input
#[derive(PartialEq, Clone)]
pub enum FocusArea {
    Roster,
    Buildings,
    PartyFormation,  // New: forming a party for a mission
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
    pub viewing_deck: bool,
    /// Current party being formed
    pub forming_party: Party,
}

impl BaseState {
    pub fn update(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) -> Option<StateTransition> {
        // Deck View handling
        if self.viewing_deck {
            if is_key_pressed(KeyCode::Escape) {
                self.viewing_deck = false;
            }
            return None; // Block other inputs while viewing deck
        }
        
        // Toggle focus (not in party formation mode)
        if is_key_pressed(KeyCode::Tab) && self.focus != FocusArea::PartyFormation {
            self.focus = match self.focus {
                FocusArea::Roster => FocusArea::Buildings,
                FocusArea::Buildings => FocusArea::Roster,
                FocusArea::PartyFormation => FocusArea::PartyFormation, // Shouldn't happen
            };
            self.viewing_deck = false;
        }
        
        match self.focus {
            FocusArea::Roster => {
                // Roster layout constants (must match draw)
                let roster_x = 300.0;
                let roster_y = 120.0;
                let card_height = 60.0;
                let card_width = 350.0;
                
                // Keyboard and mouse selection
                let available_count = roster.adventurers.len().min(9);
                for i in 0..available_count {
                    let key = match i {
                        0 => KeyCode::Key1, 1 => KeyCode::Key2, 2 => KeyCode::Key3,
                        3 => KeyCode::Key4, 4 => KeyCode::Key5, 5 => KeyCode::Key6,
                        6 => KeyCode::Key7, 7 => KeyCode::Key8, 8 => KeyCode::Key9,
                        _ => continue,
                    };
                    
                    // Keyboard selection
                    if is_key_pressed(key) {
                        self.selected_adventurer = Some(i);
                        self.selected_building = None;
                    }
                    
                    // Mouse click on adventurer card
                    let card_y = roster_y + 35.0 + (i as f32 * 70.0) - 15.0;
                    if crate::ui::was_clicked(roster_x, card_y, card_width, card_height) {
                        if self.selected_adventurer == Some(i) {
                            // Double-click = start mission
                            let adventurer = &roster.adventurers[i];
                            self.forming_party = Party::with_leader(&adventurer.id);
                            self.focus = FocusArea::PartyFormation;
                        } else {
                            self.selected_adventurer = Some(i);
                            self.selected_building = None;
                        }
                    }
                }
                
                // Roster Actions
                if let Some(adv_idx) = self.selected_adventurer {
                    if adv_idx < roster.adventurers.len() {
                        // D: Deck
                        if is_key_pressed(KeyCode::D) {
                            self.viewing_deck = true;
                        }
                        
                        // M: Start Party Formation (leader selected)
                        if is_key_pressed(KeyCode::M) {
                            let adventurer = &roster.adventurers[adv_idx];
                            self.forming_party = Party::with_leader(&adventurer.id);
                            self.focus = FocusArea::PartyFormation;
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
                // Building layout constants (must match draw)
                let build_x = 700.0;
                let build_y = 120.0;
                let card_height = 50.0;
                let card_width = 260.0;
                
                // Building selection
                let count = kingdom.buildings.len().min(9);
                for i in 0..count {
                    let key = match i {
                        0 => KeyCode::Key1, 1 => KeyCode::Key2, 2 => KeyCode::Key3,
                        3 => KeyCode::Key4, 4 => KeyCode::Key5, 5 => KeyCode::Key6,
                        6 => KeyCode::Key7, 7 => KeyCode::Key8, 8 => KeyCode::Key9,
                        _ => continue,
                    };
                    
                    // Keyboard selection
                    if is_key_pressed(key) {
                        self.selected_building = Some(i);
                        self.selected_adventurer = None;
                    }
                    
                    // Mouse click on building card
                    let card_y = build_y + 25.0 + (i as f32 * 55.0) - 10.0;
                    if crate::ui::was_clicked(build_x, card_y, card_width, card_height) {
                        if self.selected_building == Some(i) {
                            // Click on already selected = try to construct
                            self.try_construct_building(kingdom, i);
                        } else {
                            self.selected_building = Some(i);
                            self.selected_adventurer = None;
                        }
                    }
                }
                
                // Construction with Enter key
                if is_key_pressed(KeyCode::Enter) {
                    if let Some(idx) = self.selected_building {
                        self.try_construct_building(kingdom, idx);
                    }
                }
            },
            FocusArea::PartyFormation => {
                // Party Formation Mode
                // Roster layout constants (must match draw)
                let roster_x = 300.0;
                let roster_y = 120.0;
                let card_height = 60.0;
                let card_width = 350.0;
                
                // Number keys or mouse clicks add/remove members (toggle)
                let available_count = roster.adventurers.len().min(9);
                for i in 0..available_count {
                    let key = match i {
                        0 => KeyCode::Key1, 1 => KeyCode::Key2, 2 => KeyCode::Key3,
                        3 => KeyCode::Key4, 4 => KeyCode::Key5, 5 => KeyCode::Key6,
                        6 => KeyCode::Key7, 7 => KeyCode::Key8, 8 => KeyCode::Key9,
                        _ => continue,
                    };
                    
                    // Check both keyboard and mouse
                    let card_y = roster_y + 35.0 + (i as f32 * 70.0) - 15.0;
                    let card_clicked = crate::ui::was_clicked(roster_x, card_y, card_width, card_height);
                    
                    if is_key_pressed(key) || card_clicked {
                        if let Some(adv) = roster.adventurers.get(i) {
                            let id = &adv.id;
                            if self.forming_party.contains(id) {
                                // Can't remove the leader (first member)
                                if self.forming_party.leader_id() != Some(id.as_str()) {
                                    self.forming_party.remove_member(id);
                                }
                            } else if !self.forming_party.is_full() {
                                self.forming_party.add_member(id);
                            }
                        }
                    }
                }
                
                // ENTER: Launch mission with current party
                if is_key_pressed(KeyCode::Enter) && !self.forming_party.is_empty() {
                    let select = MissionSelectState::for_party(self.forming_party.clone(), roster);
                    return Some(StateTransition::ToMissionSelect(select));
                }
                
                // ESC: Cancel party formation
                if is_key_pressed(KeyCode::Escape) {
                    self.forming_party = Party::default();
                    self.focus = FocusArea::Roster;
                }
            }
        }
        
        // Global Actions (not in party formation)
        if self.focus != FocusArea::PartyFormation {
            // R to recruit (Unlocks with Guild Hall)
            if is_key_pressed(KeyCode::R) {
                let has_guild = kingdom.buildings.iter().any(|b| b.id == "guild_hall" && b.built);
                if has_guild {
                    return Some(StateTransition::ToRecruit);
                }
            }
        }
        
        None
    }
    
    /// Try to construct a building at the given index
    fn try_construct_building(&mut self, kingdom: &mut KingdomState, idx: usize) {
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
    
    pub fn draw(&self, kingdom: &KingdomState, roster: &Roster, textures: &std::collections::HashMap<String, Texture2D>) {
        // Draw title
        draw_text("FRONTIER KINGDOM", 20.0, 40.0, 32.0, WHITE);
        draw_text("Manage your kingdom. [TAB] Switch Focus", 20.0, 70.0, 18.0, GRAY);
        
        // ... (Kingdom stats drawing unchanged) ...
        let stats = &kingdom.stats;
        let y_start = 120.0;
        draw_text(&format!("Gold: {}", stats.gold), 20.0, y_start, 20.0, YELLOW);
        draw_text(&format!("Supplies: {}", stats.supplies), 20.0, y_start + 25.0, 20.0, YELLOW);
        draw_text(&format!("Security: {}", stats.security), 20.0, y_start + 50.0, 20.0, SKYBLUE);
        draw_text(&format!("Morale: {}", stats.morale), 20.0, y_start + 75.0, 20.0, ORANGE);
        draw_text(&format!("Knowledge: {}", stats.knowledge), 20.0, y_start + 100.0, 20.0, PURPLE);
        
        // Draw selected adventurer image large
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
        let card_width = 350.0;
        let card_height = 60.0;
        
        let roster_color = if self.focus == FocusArea::Roster || self.focus == FocusArea::PartyFormation { 
            YELLOW 
        } else { 
            WHITE 
        };
        let panel_title = if self.focus == FocusArea::PartyFormation {
            format!("SELECT PARTY ({}/{})", self.forming_party.size(), crate::kingdom::MAX_PARTY_SIZE)
        } else {
            "ADVENTURERS".to_string()
        };
        draw_text(&panel_title, roster_x, roster_y, 24.0, roster_color);
        
        for (i, adv) in roster.adventurers.iter().enumerate() {
            let y = roster_y + 35.0 + (i as f32 * 70.0);
            let card_y = y - 15.0;
            let is_selected = self.selected_adventurer == Some(i) && self.focus == FocusArea::Roster;
            let is_hovered = crate::ui::is_mouse_over(roster_x, card_y, card_width, card_height);
            let is_in_party = self.focus == FocusArea::PartyFormation && self.forming_party.contains(&adv.id);
            let is_leader = self.focus == FocusArea::PartyFormation && self.forming_party.leader_id() == Some(&adv.id);
            
            // Card background - different colors for party selection + hover
            let bg_color = if is_leader {
                Color::from_rgba(80, 100, 60, 255) // Leader = yellowish-green
            } else if is_in_party {
                Color::from_rgba(60, 80, 100, 255) // In party = blue-ish
            } else if is_selected {
                Color::from_rgba(60, 80, 60, 255)
            } else if is_hovered {
                Color::from_rgba(50, 55, 60, 255) // Hover highlight
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(roster_x, card_y, card_width, card_height, bg_color);
            
            // Border for selected/party members/hovered
            if is_leader {
                draw_rectangle_lines(roster_x, card_y, card_width, card_height, 3.0, YELLOW);
            } else if is_in_party {
                draw_rectangle_lines(roster_x, card_y, card_width, card_height, 2.0, SKYBLUE);
            } else if is_selected {
                draw_rectangle_lines(roster_x, card_y, card_width, card_height, 2.0, GREEN);
            } else if is_hovered {
                draw_rectangle_lines(roster_x, card_y, card_width, card_height, 1.0, LIGHTGRAY);
            }
            
            // Adventurer info
            let name_color = if is_leader { YELLOW } else if is_in_party { SKYBLUE } else if is_selected { GREEN } else { WHITE };
            let leader_mark = if is_leader { "★ " } else { "" };
            let party_mark = if is_in_party && !is_leader { "✓ " } else { "" };
            draw_text(&format!("[{}] {}{}{}", i + 1, leader_mark, party_mark, adv.name), roster_x + 10.0, y + 5.0, 20.0, name_color);
            
            // Class and deck size (base deck is ~5 cards per class + additions)
            let base_deck_size = 5; // Class cards + Any cards
            let deck_size = base_deck_size + adv.deck_additions.len();
            draw_text(&format!("{:?} • {} cards", adv.class, deck_size), roster_x + 200.0, y + 5.0, 14.0, GRAY);
            
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
        
        // Draw description for selected building
        if self.focus == FocusArea::Buildings {
            if let Some(idx) = self.selected_building {
                if let Some(b) = kingdom.buildings.get(idx) {
                    let desc_y = build_y + (kingdom.buildings.len() as f32 * 70.0) + 50.0;
                    draw_text("BUILDING INFO:", build_x, desc_y, 20.0, YELLOW);
                    
                    let max_width = 40;
                    let mut current_line = String::new();
                    let mut line_y = desc_y + 25.0;
                    
                    for word in b.description.split_whitespace() {
                        if current_line.len() + word.len() > max_width {
                            draw_text(&current_line, build_x, line_y, 18.0, WHITE);
                            current_line.clear();
                            line_y += 20.0;
                        }
                        current_line.push_str(word);
                        current_line.push(' ');
                    }
                    if !current_line.is_empty() {
                        draw_text(&current_line, build_x, line_y, 18.0, WHITE);
                    }
                }
            }
        }
        
        // DECK VIEWER OVERLAY
        if self.viewing_deck {
            if let Some(idx) = self.selected_adventurer {
                if let Some(adv) = roster.adventurers.get(idx) {
                    // Draw dimming overlay
                    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 220));
                    
                    draw_text(&format!("{}'s Deck", adv.name), 50.0, 50.0, 40.0, WHITE);
                    draw_text("[ESC] Close", screen_width() - 150.0, 50.0, 20.0, GRAY);
                    
                    // Reconstruct deck
                    // Note: This is inefficient to do every frame, but fine for simple prototype
                    let mut deck = crate::data::cards::load_starter_deck().unwrap_or_default();
                    if let Ok(all_cards) = crate::data::cards::CardData::load_all() {
                        for id in &adv.deck_additions {
                            if let Some(data) = all_cards.iter().find(|c| c.id == *id) {
                                deck.push(data.to_card());
                            }
                        }
                    }
                    
                    // Draw Cards Grid
                    let start_x = 50.0;
                    let start_y = 100.0;
                    let card_w = 120.0;
                    let card_h = 160.0;
                    let gap = 20.0;
                    let cols = ((screen_width() - 100.0) / (card_w + gap)) as i32;
                    
                    for (i, card) in deck.iter().enumerate() {
                        let row = (i as i32) / cols;
                        let col = (i as i32) % cols;
                        
                        let x = start_x + (col as f32 * (card_w + gap));
                        let y = start_y + (row as f32 * (card_h + gap));
                        
                        // Card Body
                        draw_rectangle(x, y, card_w, card_h, DARKGRAY);
                        draw_rectangle_lines(x, y, card_w, card_h, 2.0, WHITE);
                        
                        // Card Info
                        draw_text(&card.name, x + 5.0, y + 20.0, 16.0, WHITE);
                        draw_text(&format!("Cost: {}", card.cost), x + 5.0, y + 40.0, 16.0, SKYBLUE);
                        
                        // Description (Tiny)
                        // Simple word wrap
                        let mut desc_y = y + 60.0;
                         let max_chars = 15;
                        let mut current_line = String::new();
                        for word in card.description.split_whitespace() {
                            if current_line.len() + word.len() > max_chars {
                                draw_text(&current_line, x + 5.0, desc_y, 14.0, LIGHTGRAY);
                                current_line.clear();
                                desc_y += 14.0;
                            }
                            current_line.push_str(word);
                            current_line.push(' ');
                        }
                        if !current_line.is_empty() {
                            draw_text(&current_line, x + 5.0, desc_y, 14.0, LIGHTGRAY);
                        }
                    }
                }
            }
            return; // Skip drawing main instructions if overlay active
        }
        
        // Draw instructions
        let instruction = match self.focus {
            FocusArea::Roster => {
                if self.selected_adventurer.is_some() {
                    let mut s = String::from("[M] Launch Mission  [D] View Deck");
                    let has_infirmary = kingdom.buildings.iter().any(|b| b.id == "infirmary" && b.built);
                    let has_chapel = kingdom.buildings.iter().any(|b| b.id == "chapel" && b.built);
                    let has_guild = kingdom.buildings.iter().any(|b| b.id == "guild_hall" && b.built);
                    
                    if has_infirmary { s.push_str("  [H] Heal(10s)"); }
                    if has_chapel { s.push_str("  [T] Tavern(10s)"); }
                    if has_guild { s.push_str("  [R] Recruit"); }
                    
                    s
                } else {
                    "[1-9] Select Adventurer to view Deck, start Mission, or manage".to_string()
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
            },
            FocusArea::PartyFormation => {
                format!(
                    "FORMING PARTY ({}/{})  [1-9] Add/Remove Member  [ENTER] Launch Mission  [ESC] Cancel",
                    self.forming_party.size(),
                    crate::kingdom::MAX_PARTY_SIZE
                )
            }
        };
        
        draw_text(&instruction, 20.0, screen_height() - 40.0, 20.0, GREEN);
        draw_text("[F5] Save  [F9] Load  [TAB] Switch Focus", 20.0, screen_height() - 20.0, 16.0, GRAY);
    }
}
