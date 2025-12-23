//! Combat state - turn-based card combat

use macroquad::prelude::*;
use super::{StateTransition, ResultState, MissionState};
use crate::combat::{Unit, Card, CombatResolver};
use crate::missions::Mission;
use crate::data::random_enemy_for_difficulty;

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
    /// Track damage/stress for applying to adventurer after combat
    pub damage_taken: i32,
    pub stress_gained: i32,
}

/// Context needed to return to a mission after combat
#[derive(Clone)]
pub struct MissionContext {
    pub mission: Mission,
    pub current_node: usize,
    pub adventurer_id: String,
    pub adventurer_name: String,
    pub adventurer_hp: i32,
    pub adventurer_max_hp: i32,
    pub adventurer_stress: i32,
    pub adventurer_image: Option<String>,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            player: Unit::new_player("Adventurer", 50),
            enemy: Unit::new_enemy("Forest Beast", 30, None),
            hand: Card::starter_hand(),
            energy: 3,
            max_energy: 3,
            turn: 1,
            selected_card: None,
            resolver: CombatResolver::new(),
            adventurer_id: String::new(),
            return_mission: None,
            damage_taken: 0,
            stress_gained: 0,
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
    
    /// Create combat that returns to mission on victory, using real adventurer stats
    pub fn for_mission(context: MissionContext) -> Self {
        // Use adventurer's actual HP
        let mut player = Unit::new_player(&context.adventurer_name, context.adventurer_max_hp);
        player.hp = context.adventurer_hp;
        player.stress = context.adventurer_stress;
        player.image_path = context.adventurer_image.clone();
        
        // Get random enemy based on mission difficulty
        let enemy = random_enemy_for_difficulty(context.mission.difficulty);
        
        Self {
            player,
            enemy,
            adventurer_id: context.adventurer_id.clone(),
            return_mission: Some(context),
            damage_taken: 0,
            stress_gained: 0,
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
                // Create updated context with current HP/stress
                let mut updated_ctx = ctx.clone();
                updated_ctx.adventurer_hp = self.player.hp;
                updated_ctx.adventurer_stress = self.player.stress;
                
                let mission_state = MissionState::from_mission_with_stats(
                    updated_ctx.mission.clone(),
                    updated_ctx.adventurer_id.clone(),
                    updated_ctx.adventurer_name.clone(),
                    updated_ctx.adventurer_hp,
                    updated_ctx.adventurer_max_hp,
                    updated_ctx.adventurer_stress,
                    updated_ctx.adventurer_image.clone(),
                ).with_node(ctx.current_node);
                return Some(StateTransition::ToMission(mission_state));
            } else {
                return Some(StateTransition::ToResults(ResultState::victory_for(&self.adventurer_id)));
            }
        }
        if self.player.hp <= 0 {
            // Defeat - always go to results
            let mut results = ResultState::defeat_for(&self.adventurer_id);
            results.hp_lost = self.damage_taken;
            results.stress_gained = self.stress_gained;
            results.final_hp = Some(self.player.hp);
            return Some(StateTransition::ToResults(results));
        }
        
        None
    }
    
    fn end_turn(&mut self) {
        // Player status tick
        self.player.tick_statuses();
        self.player.block = 0;
        
        // Enemy Action
        let (dmg, stress) = self.enemy.execute_intent();
        
        // Apply damage to player
        if dmg > 0 {
            let actual = self.player.take_damage(dmg);
            self.damage_taken += actual;
        }
        
        self.player.add_stress(stress);
        self.stress_gained += 2 + stress;
        
        // Enemy status tick
        self.enemy.tick_statuses();
        self.enemy.block = 0;
        
        // Next Turn
        self.turn += 1;
        self.energy = self.max_energy;
        self.hand = Card::starter_hand(); // TODO: Draw from Deck
        
        // Roll new enemy intent for next turn
        self.enemy.roll_intent(self.turn);
    }
    
    pub fn draw(&self, textures: &std::collections::HashMap<String, Texture2D>) {
        // Draw background
        let region_id = if let Some(ctx) = &self.return_mission {
            &ctx.mission.region_id
        } else {
            "dark_woods"
        };
        
        let bg_path = format!("assets/images/regions/{}.png", region_id);
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
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 180));
        }
        
        // Combat header
        draw_text("COMBAT", 20.0, 40.0, 28.0, RED);
        draw_text(&format!("Turn {}", self.turn), 20.0, 70.0, 20.0, YELLOW);
        
        // Player stats
        let player_y = 120.0;
        draw_text(&self.player.name, 20.0, player_y, 22.0, WHITE);
        draw_text(&format!("HP: {}/{}", self.player.hp, self.player.max_hp), 20.0, player_y + 25.0, 18.0, GREEN);
        draw_text(&format!("Block: {}", self.player.block), 20.0, player_y + 45.0, 18.0, BLUE);
        draw_text(&format!("Stress: {}", self.player.stress), 20.0, player_y + 65.0, 18.0, ORANGE);
        
        // Draw Player Statuses
        let mut sx = 20.0;
        let sy = player_y + 85.0; // Draw above image or over it? Let's verify space.
        // Image is at player_y + 80? Wait, let's shift image down or draw statuses next to name.
        // Let's draw statuses below stats.
        for status in &self.player.statuses {
             let color = match status.effect_type {
                 crate::kingdom::StatusType::Vulnerable | crate::kingdom::StatusType::Weak | crate::kingdom::StatusType::Stun => RED,
                 _ => GREEN,
             };
             let text = format!("{:?}({})", status.effect_type, status.duration);
             draw_text(&text, sx, sy, 16.0, color);
             sx += 100.0; // Spacing
        }
        
        // Player image
        if let Some(path) = &self.player.image_path {
            if let Some(tex) = textures.get(path) {
                draw_texture_ex(
                    tex,
                    20.0, player_y + 110.0, // Shifted down
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(120.0, 120.0)),
                        ..Default::default()
                    }
                );
            }
        }
        
        // Energy (moved below image to avoid overlap)
        draw_text(&format!("Energy: {}/{}", self.energy, self.max_energy), 20.0, player_y + 240.0, 20.0, SKYBLUE);
        
        // Enemy stats
        let enemy_x = screen_width() - 200.0;
        draw_text(&self.enemy.name, enemy_x, player_y, 22.0, RED);
        draw_text(&format!("HP: {}/{}", self.enemy.hp, self.enemy.max_hp), enemy_x, player_y + 25.0, 18.0, GREEN);
        draw_text(&format!("Block: {}", self.enemy.block), enemy_x, player_y + 45.0, 18.0, BLUE);
        
        // Draw Enemy Statuses
        let mut ex = enemy_x;
        let ey = player_y + 65.0;
        for status in &self.enemy.statuses {
             let color = match status.effect_type {
                 crate::kingdom::StatusType::Vulnerable | crate::kingdom::StatusType::Weak | crate::kingdom::StatusType::Stun => GREEN, // Good for player
                 _ => RED,
             };
             let text = format!("{:?}({})", status.effect_type, status.duration);
             draw_text(&text, ex, ey, 16.0, color);
             ex += 100.0; // Wrap?
             // Since right aligned, this might go off screen. 
             // Let's stack vertically.
        }
        
        // Enemy intent - shows what they'll do next
        let intent_color = match &self.enemy.intent {
            crate::combat::EnemyIntent::Attack(_) => RED,
            crate::combat::EnemyIntent::Block(_) => BLUE,
            crate::combat::EnemyIntent::Buff => YELLOW,
            crate::combat::EnemyIntent::Debuff => PURPLE,
            crate::combat::EnemyIntent::Unknown => GRAY,
        };
        draw_text(&format!("Intent: {}", self.enemy.intent.description()), enemy_x, player_y + 65.0, 16.0, intent_color);
        
        // Enemy image
        if let Some(path) = &self.enemy.image_path {
            if let Some(tex) = textures.get(path) {
                draw_texture_ex(
                    tex,
                    enemy_x, player_y + 85.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(120.0, 120.0)),
                        ..Default::default()
                    }
                );
            }
        }
        
        
        // Hand of cards
        let card_y = screen_height() - 250.0;
        let card_width = 140.0;
        let card_height = 160.0;
        
        for (i, card) in self.hand.iter().enumerate() {
            let x = 20.0 + (i as f32 * (card_width + 10.0));
            let is_selected = self.selected_card == Some(i);
            let can_play = card.cost <= self.energy;
            
            // Card background/border
            let border_color = if is_selected {
                YELLOW
            } else if can_play {
                WHITE
            } else {
                DARKGRAY
            };
            draw_rectangle(x - 2.0, card_y - 2.0, card_width + 4.0, card_height + 4.0, border_color);
            
            // Card image
            if let Some(path) = &card.image_path {
                if let Some(tex) = textures.get(path) {
                    draw_texture_ex(
                        tex,
                        x, card_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(card_width, card_height - 40.0)),
                            ..Default::default()
                        }
                    );
                } else {
                    // Fallback: draw colored rectangle
                    let bg_color = Color::from_rgba(60, 60, 80, 255);
                    draw_rectangle(x, card_y, card_width, card_height - 40.0, bg_color);
                }
            } else {
                let bg_color = Color::from_rgba(60, 60, 80, 255);
                draw_rectangle(x, card_y, card_width, card_height - 40.0, bg_color);
            }
            
            // Card info bar at bottom
            let info_y = card_y + card_height - 40.0;
            draw_rectangle(x, info_y, card_width, 40.0, Color::from_rgba(20, 20, 30, 240));
            
            // Card text
            let text_color = WHITE;
            draw_text(&format!("[{}] {}", i + 1, card.name), x + 5.0, info_y + 15.0, 14.0, text_color);
            draw_text(&format!("Cost: {}", card.cost), x + 5.0, info_y + 32.0, 12.0, 
                if can_play { GREEN } else { RED });
        }
        
        // Instructions
        draw_text("[1-5] Select Card  [ENTER] Play  [E] End Turn", 20.0, screen_height() - 30.0, 18.0, GREEN);
    }
}
