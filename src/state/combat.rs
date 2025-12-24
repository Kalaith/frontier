//! Combat state - turn-based card combat

use macroquad::prelude::*;
use super::{StateTransition, ResultState, MissionState};
use crate::combat::{Unit, Card, CombatResolver};
use crate::missions::Mission;
use crate::kingdom::PartyMemberState;
use crate::data::random_enemy_for_difficulty;

/// Turn-based combat state with party support
pub struct CombatState {
    /// All player units (party members)
    pub players: Vec<Unit>,
    /// Index of the currently active player
    pub current_player_idx: usize,
    pub enemy: Unit,
    pub hand: Vec<Card>,
    pub energy: i32,
    pub max_energy: i32,
    pub turn: usize,
    pub selected_card: Option<usize>,
    pub resolver: CombatResolver,
    /// Mission to return to after combat victory
    pub return_mission: Option<MissionContext>,
    /// Track damage/stress per player for applying after combat
    pub damage_taken: Vec<i32>,
    pub stress_gained: Vec<i32>,
}

/// Context needed to return to a mission after combat
#[derive(Clone)]
pub struct MissionContext {
    pub mission: Mission,
    pub current_node: usize,
    pub party_members: Vec<PartyMemberState>,
}

impl MissionContext {
    /// Get the leader
    #[allow(dead_code)]
    pub fn leader(&self) -> Option<&PartyMemberState> {
        self.party_members.first()
    }
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            players: vec![Unit::new_player("Adventurer", 50)],
            current_player_idx: 0,
            enemy: Unit::new_enemy("Forest Beast", 30, None),
            hand: Card::starter_hand(),
            energy: 3,
            max_energy: 3,
            turn: 1,
            selected_card: None,
            resolver: CombatResolver::new(),
            return_mission: None,
            damage_taken: vec![0],
            stress_gained: vec![0],
        }
    }
}

impl CombatState {
    /// Get the currently active player
    #[allow(dead_code)]
    pub fn current_player(&self) -> Option<&Unit> {
        self.players.get(self.current_player_idx)
    }
    
    /// Get the currently active player mutably
    #[allow(dead_code)]
    pub fn current_player_mut(&mut self) -> Option<&mut Unit> {
        self.players.get_mut(self.current_player_idx)
    }
    
    /// Create combat state for a specific adventurer (backwards compat)
    #[allow(dead_code)]
    pub fn for_adventurer(_adventurer_id: &str, adventurer_name: &str) -> Self {
        let player = Unit::new_player(adventurer_name, 50);
        Self {
            players: vec![player],
            ..Default::default()
        }
    }
    
    /// Create combat that returns to mission on victory, using party stats
    pub fn for_mission(context: MissionContext) -> Self {
        // Create Unit for each party member
        let players: Vec<Unit> = context.party_members.iter().map(|m| {
            let mut unit = Unit::new_player(&m.name, m.max_hp);
            unit.hp = m.hp;
            unit.stress = m.stress;
            unit.image_path = m.image_path.clone();
            unit
        }).collect();
        
        let party_size = players.len();
        
        // Get random enemy based on mission difficulty
        let enemy = random_enemy_for_difficulty(context.mission.difficulty);
        
        Self {
            players,
            current_player_idx: 0,
            enemy,
            return_mission: Some(context),
            damage_taken: vec![0; party_size],
            stress_gained: vec![0; party_size],
            ..Default::default()
        }
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Card layout constants (must match draw)
        let card_y = screen_height() - 250.0;
        let card_width = 140.0;
        let card_height = 160.0;
        
        // Card selection with number keys OR mouse click
        for i in 0..self.hand.len().min(5) {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                _ => continue,
            };
            
            // Keyboard selection
            if is_key_pressed(key) {
                self.selected_card = Some(i);
            }
            
            // Mouse click on card
            let card_x = 20.0 + (i as f32 * (card_width + 10.0));
            if crate::ui::was_clicked(card_x, card_y, card_width, card_height) {
                if self.selected_card == Some(i) {
                    // Clicking already selected card = play it
                    self.try_play_selected_card();
                } else {
                    self.selected_card = Some(i);
                }
            }
        }
        
        // Play selected card with Enter
        if is_key_pressed(KeyCode::Enter) {
            self.try_play_selected_card();
        }
        
        // End turn with E key or button click (button drawn in draw())
        if is_key_pressed(KeyCode::E) {
            self.end_turn();
        }
        // End Turn button bounds
        let end_btn_x = screen_width() - 150.0;
        let end_btn_y = screen_height() - 60.0;
        if crate::ui::was_clicked(end_btn_x, end_btn_y, 130.0, 40.0) {
            self.end_turn();
        }
        
        // Check win/lose
        if self.enemy.hp <= 0 {
            // Victory - return to mission if we came from one
            if let Some(ctx) = &self.return_mission {
                // Update party member states with current HP/stress
                let updated_members: Vec<PartyMemberState> = self.players.iter().map(|p| {
                    PartyMemberState {
                        id: ctx.party_members.iter().find(|m| m.name == p.name).map(|m| m.id.clone()).unwrap_or_default(),
                        name: p.name.clone(),
                        hp: p.hp,
                        max_hp: p.max_hp,
                        stress: p.stress,
                        image_path: p.image_path.clone(),
                    }
                }).collect();
                
                let mission_state = MissionState::from_mission_with_party(
                    ctx.mission.clone(),
                    updated_members,
                ).with_node(ctx.current_node);
                return Some(StateTransition::ToMission(mission_state));
            } else {
                // Not from mission - just show simple victory
                let leader_id = self.players.first().map(|p| p.name.as_str()).unwrap_or("");
                return Some(StateTransition::ToResults(ResultState::victory_for(leader_id)));
            }
        }
        
        // Check if all players are dead
        let all_dead = self.players.iter().all(|p| p.hp <= 0);
        if all_dead {
            // Defeat - always go to results
            if let Some(ctx) = &self.return_mission {
                let results = ResultState::defeat_for_party(&ctx.party_members);
                return Some(StateTransition::ToResults(results));
            } else {
                let leader_id = self.players.first().map(|p| p.name.as_str()).unwrap_or("");
                let results = ResultState::defeat_for(leader_id);
                return Some(StateTransition::ToResults(results));
            }
        }
        
        None
    }
    
    /// Try to play the currently selected card
    fn try_play_selected_card(&mut self) {
        if let Some(card_idx) = self.selected_card {
            if card_idx < self.hand.len() && self.current_player_idx < self.players.len() {
                let card = &self.hand[card_idx];
                
                // Check if card can be played
                let can_afford = card.cost <= self.energy;
                let attack_blocked = card.is_attack() && self.resolver.turn_mods.attacks_disabled;
                
                if can_afford && !attack_blocked {
                    // Resolve card effects
                    let effects = card.effects.clone();
                    self.energy -= card.cost;
                    
                    // Get active player reference
                    let player = &mut self.players[self.current_player_idx];
                    for effect in effects {
                        self.resolver.resolve(&effect, player, &mut self.enemy);
                    }
                    
                    self.hand.remove(card_idx);
                    self.selected_card = None;
                }
            }
        }
    }
    
    fn end_turn(&mut self) {
        // Current player status tick and block reset
        if let Some(player) = self.players.get_mut(self.current_player_idx) {
            player.tick_statuses();
            player.block = 0;
        }
        
        // Enemy Action
        let (dmg, stress) = self.enemy.execute_intent();
        let enemy_acted = dmg > 0 || stress > 0;
        
        // Apply damage to current player
        if dmg > 0 {
            if let Some(player) = self.players.get_mut(self.current_player_idx) {
                let actual = player.take_damage(dmg);
                if self.current_player_idx < self.damage_taken.len() {
                    self.damage_taken[self.current_player_idx] += actual;
                }
            }
        }
        
        // Apply stress with resistance (uses resolver's turn mods)
        let base_stress = 2 + stress;
        if let Some(player) = self.players.get_mut(self.current_player_idx) {
            self.resolver.apply_stress_to_player(player, base_stress);
            if self.current_player_idx < self.stress_gained.len() {
                self.stress_gained[self.current_player_idx] += base_stress;
            }
        }
        
        // Enemy status tick
        self.enemy.tick_statuses();
        self.enemy.block = 0;
        
        // Reset turn modifiers and track enemy action for next turn
        self.resolver.end_turn(enemy_acted);
        
        // Cycle to next living party member
        if self.players.len() > 1 {
            let start_idx = self.current_player_idx;
            loop {
                self.current_player_idx = (self.current_player_idx + 1) % self.players.len();
                // Stop if alive or back to start
                if self.players[self.current_player_idx].hp > 0 || self.current_player_idx == start_idx {
                    break;
                }
            }
        }
        
        // Next Turn
        self.turn += 1;
        self.energy = self.max_energy;
        self.hand = Card::starter_hand(); // Draws from JSON-loaded deck
        
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
        
        // Party member portraits (if more than 1)
        let portrait_y = 95.0;
        let portrait_size = 50.0;
        for (i, player) in self.players.iter().enumerate() {
            let x = 20.0 + (i as f32 * (portrait_size + 10.0));
            let is_active = i == self.current_player_idx;
            
            // Portrait border
            let border_color = if is_active { YELLOW } else if player.hp <= 0 { RED } else { GRAY };
            draw_rectangle_lines(x - 2.0, portrait_y - 2.0, portrait_size + 4.0, portrait_size + 4.0, 2.0, border_color);
            
            // Portrait
            if let Some(path) = &player.image_path {
                if let Some(tex) = textures.get(path) {
                    let tint = if player.hp <= 0 { Color::from_rgba(100, 100, 100, 255) } else { WHITE };
                    draw_texture_ex(
                        tex,
                        x, portrait_y,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(vec2(portrait_size, portrait_size)),
                            ..Default::default()
                        }
                    );
                }
            }
            
            // HP bar under portrait
            let hp_pct = (player.hp as f32 / player.max_hp as f32).max(0.0);
            draw_rectangle(x, portrait_y + portrait_size + 2.0, portrait_size, 6.0, DARKGRAY);
            draw_rectangle(x, portrait_y + portrait_size + 2.0, portrait_size * hp_pct, 6.0, GREEN);
        }
        
        // Current player stats (detailed)
        let player_y = 180.0;
        if let Some(player) = self.players.get(self.current_player_idx) {
            draw_text(&format!("Active: {}", player.name), 20.0, player_y, 22.0, YELLOW);
            draw_text(&format!("HP: {}/{}", player.hp, player.max_hp), 20.0, player_y + 25.0, 18.0, GREEN);
            draw_text(&format!("Block: {}", player.block), 20.0, player_y + 45.0, 18.0, BLUE);
            draw_text(&format!("Stress: {}", player.stress), 20.0, player_y + 65.0, 18.0, ORANGE);
            
            // Draw Player Statuses
            let mut sx = 20.0;
            let sy = player_y + 85.0;
            for status in &player.statuses {
                let color = match status.effect_type {
                    crate::kingdom::StatusType::Vulnerable | crate::kingdom::StatusType::Weak | crate::kingdom::StatusType::Stun => RED,
                    _ => GREEN,
                };
                let text = format!("{:?}({})", status.effect_type, status.duration);
                draw_text(&text, sx, sy, 16.0, color);
                sx += 100.0;
            }
        }
        
        // Energy
        draw_text(&format!("Energy: {}/{}", self.energy, self.max_energy), 20.0, player_y + 105.0, 20.0, SKYBLUE);
        
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
            let is_hovered = crate::ui::is_mouse_over(x, card_y, card_width, card_height);
            let can_afford = card.cost <= self.energy;
            let attack_blocked = card.is_attack() && self.resolver.turn_mods.attacks_disabled;
            let can_play = can_afford && !attack_blocked;
            
            // Card background/border - add hover glow
            let border_color = if is_selected {
                if can_play { YELLOW } else { ORANGE }
            } else if is_hovered && can_play {
                Color::from_rgba(150, 255, 150, 255) // Green glow on hover
            } else if can_play {
                WHITE
            } else if attack_blocked {
                PURPLE  // Visual cue for blocked attacks
            } else {
                DARKGRAY
            };
            
            // Draw slightly larger border if hovered
            let border_thickness = if is_hovered || is_selected { 4.0 } else { 2.0 };
            draw_rectangle(x - border_thickness, card_y - border_thickness, 
                          card_width + border_thickness * 2.0, card_height + border_thickness * 2.0, 
                          border_color);
            
            // Card image
            if let Some(path) = &card.image_path {
                if let Some(tex) = textures.get(path) {
                    draw_texture_ex(
                        tex,
                        x, card_y,
                        if attack_blocked { Color::from_rgba(150, 100, 150, 255) } else { WHITE },
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
            let text_color = if attack_blocked { PURPLE } else { WHITE };
            draw_text(&format!("[{}] {}", i + 1, card.name), x + 5.0, info_y + 15.0, 14.0, text_color);
            
            // Cost and status
            let status_text = if attack_blocked {
                "BLOCKED"
            } else {
                &format!("Cost: {}", card.cost)
            };
            let status_color = if attack_blocked { PURPLE } else if can_afford { GREEN } else { RED };
            draw_text(status_text, x + 5.0, info_y + 32.0, 12.0, status_color);
        }
        
        // End Turn button
        let end_btn_x = screen_width() - 150.0;
        let end_btn_y = screen_height() - 60.0;
        crate::ui::button("END TURN", end_btn_x, end_btn_y, 130.0, 40.0);
        
        // Instructions
        draw_text("Click card to select, click again to play • Click END TURN or press [E]", 20.0, screen_height() - 30.0, 16.0, GREEN);
    }
}
