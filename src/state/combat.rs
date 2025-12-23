//! Combat state - turn-based card combat

use macroquad::prelude::*;
use super::{StateTransition, ResultState, MissionState};
use crate::combat::{Unit, Card, CombatResolver};
use crate::missions::Mission;

/// Turn-based combat state
pub struct CombatState {
    pub player: Unit,
    pub enemy: Unit,
    pub hand: Vec<Card>,
    pub energy: i32,
    pub max_energy: i32,
    pub turn: usize,
    pub selected_card: Option<usize>,
    pub resolver: CombatResolver,
    pub adventurer_id: String,
    /// Mission to return to after combat victory
    pub return_mission: Option<MissionContext>,
}

/// Context needed to return to a mission after combat
#[derive(Clone)]
pub struct MissionContext {
    pub mission: Mission,
    pub current_node: usize,
    pub adventurer_id: String,
    pub adventurer_name: String,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            player: Unit::new_player("Adventurer", 50),
            enemy: Unit::new_enemy("Forest Beast", 30),
            hand: Card::starter_hand(),
            energy: 3,
            max_energy: 3,
            turn: 1,
            selected_card: None,
            resolver: CombatResolver::new(),
            adventurer_id: String::new(),
            return_mission: None,
        }
    }
}

impl CombatState {
    /// Create combat state for a specific adventurer
    pub fn for_adventurer(adventurer_id: &str, adventurer_name: &str) -> Self {
        Self {
            player: Unit::new_player(adventurer_name, 50),
            adventurer_id: adventurer_id.to_string(),
            ..Default::default()
        }
    }
    
    /// Create combat that returns to mission on victory
    pub fn for_mission(context: MissionContext) -> Self {
        Self {
            player: Unit::new_player(&context.adventurer_name, 50),
            adventurer_id: context.adventurer_id.clone(),
            return_mission: Some(context),
            ..Default::default()
        }
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Card selection with number keys
        for i in 0..self.hand.len().min(5) {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                _ => continue,
            };
            
            if is_key_pressed(key) {
                self.selected_card = Some(i);
            }
        }
        
        // Play selected card with Enter
        if is_key_pressed(KeyCode::Enter) {
            if let Some(card_idx) = self.selected_card {
                if card_idx < self.hand.len() {
                    let card = &self.hand[card_idx];
                    if card.cost <= self.energy {
                        // Resolve card effects
                        let effects = card.effects.clone();
                        self.energy -= card.cost;
                        
                        for effect in effects {
                            self.resolver.resolve(&effect, &mut self.player, &mut self.enemy);
                        }
                        
                        self.hand.remove(card_idx);
                        self.selected_card = None;
                    }
                }
            }
        }
        
        // End turn with E
        if is_key_pressed(KeyCode::E) {
            self.end_turn();
        }
        
        // Check win/lose
        if self.enemy.hp <= 0 {
            // Victory - return to mission if we came from one
            if let Some(ctx) = &self.return_mission {
                let mission_state = MissionState::from_mission(
                    ctx.mission.clone(),
                    ctx.adventurer_id.clone(),
                    ctx.adventurer_name.clone(),
                ).with_node(ctx.current_node);
                return Some(StateTransition::ToMission(mission_state));
            } else {
                return Some(StateTransition::ToResults(ResultState::victory_for(&self.adventurer_id)));
            }
        }
        if self.player.hp <= 0 {
            // Defeat - always go to results
            return Some(StateTransition::ToResults(ResultState::defeat_for(&self.adventurer_id)));
        }
        
        None
    }
    
    fn end_turn(&mut self) {
        // Enemy attacks
        let enemy_damage = 5 + (self.turn as i32);
        let actual_damage = (enemy_damage - self.player.block).max(0);
        self.player.hp -= actual_damage;
        self.player.stress += 2;
        
        // Reset for next turn
        self.player.block = 0;
        self.turn += 1;
        self.energy = self.max_energy;
        self.hand = Card::starter_hand(); // Redraw hand
    }
    
    pub fn draw(&self) {
        // Combat header
        draw_text("COMBAT", 20.0, 40.0, 28.0, RED);
        draw_text(&format!("Turn {}", self.turn), 20.0, 70.0, 20.0, YELLOW);
        
        // Player stats
        let player_y = 120.0;
        draw_text(&self.player.name, 20.0, player_y, 22.0, WHITE);
        draw_text(&format!("HP: {}/{}", self.player.hp, self.player.max_hp), 20.0, player_y + 25.0, 18.0, GREEN);
        draw_text(&format!("Block: {}", self.player.block), 20.0, player_y + 45.0, 18.0, BLUE);
        draw_text(&format!("Stress: {}", self.player.stress), 20.0, player_y + 65.0, 18.0, ORANGE);
        
        // Enemy stats
        let enemy_x = screen_width() - 200.0;
        draw_text(&self.enemy.name, enemy_x, player_y, 22.0, RED);
        draw_text(&format!("HP: {}/{}", self.enemy.hp, self.enemy.max_hp), enemy_x, player_y + 25.0, 18.0, GREEN);
        draw_text(&format!("Block: {}", self.enemy.block), enemy_x, player_y + 45.0, 18.0, BLUE);
        
        // Energy
        draw_text(&format!("Energy: {}/{}", self.energy, self.max_energy), 20.0, player_y + 100.0, 20.0, SKYBLUE);
        
        // Hand of cards
        let card_y = screen_height() - 180.0;
        let card_width = 150.0;
        let card_height = 120.0;
        
        for (i, card) in self.hand.iter().enumerate() {
            let x = 20.0 + (i as f32 * (card_width + 10.0));
            let is_selected = self.selected_card == Some(i);
            let can_play = card.cost <= self.energy;
            
            // Card background
            let bg_color = if is_selected {
                YELLOW
            } else if can_play {
                Color::from_rgba(60, 60, 80, 255)
            } else {
                Color::from_rgba(40, 40, 50, 255)
            };
            draw_rectangle(x, card_y, card_width, card_height, bg_color);
            
            // Card content
            let text_color = if is_selected { BLACK } else { WHITE };
            draw_text(&format!("[{}]", i + 1), x + 5.0, card_y + 20.0, 16.0, text_color);
            draw_text(&card.name, x + 5.0, card_y + 45.0, 16.0, text_color);
            draw_text(&format!("Cost: {}", card.cost), x + 5.0, card_y + 65.0, 14.0, text_color);
            draw_text(&card.description, x + 5.0, card_y + 85.0, 12.0, text_color);
        }
        
        // Instructions
        draw_text("[1-5] Select Card  [ENTER] Play  [E] End Turn", 20.0, screen_height() - 30.0, 18.0, GREEN);
    }
}
