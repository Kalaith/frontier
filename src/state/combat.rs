//! Combat state - turn-based card combat

use super::{MissionState, ResultState, StateTransition};
use crate::combat::{Card, CombatResolver, Unit};
use crate::data::random_enemy_for_region_and_difficulty;
use crate::kingdom::{PartyMemberState, ResolveState, TraumaType};
use crate::missions::{MapNode, Mission};
use macroquad::prelude::*;

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
    /// Short-lived UI feedback for clicks and keyboard actions
    pub feedback: Option<(String, f32)>,
}

/// Context needed to return to a mission after combat
#[derive(Clone)]
pub struct MissionContext {
    pub mission: Mission,
    pub current_node: usize,
    pub party_members: Vec<PartyMemberState>,
    /// The generated map nodes for this mission run
    pub map_nodes: Vec<MapNode>,
    /// Nodes that have been visited
    pub visited_nodes: Vec<usize>,
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
            feedback: None,
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
        let players: Vec<Unit> = context
            .party_members
            .iter()
            .map(|m| {
                let mut unit = Unit::new_player(&m.name, m.max_hp);
                unit.hp = m.hp;
                unit.stress = m.stress;
                unit.image_path = m.image_path.clone();
                unit.traumas = m.traumas.clone();
                unit.resolve_state = m.resolve_state.clone();
                unit
            })
            .collect();

        let party_size = players.len();

        // Get the current player's class to load appropriate cards
        let class_name = context
            .party_members
            .first()
            .map(|m| m.class_name.as_str())
            .unwrap_or("Soldier");
        let deck_additions = context
            .party_members
            .first()
            .map(|m| m.deck_additions.as_slice())
            .unwrap_or(&[]);
        let hand = Card::load_deck_for_class(class_name, deck_additions)
            .into_iter()
            .take(5)
            .collect();

        // Get random enemy based on mission region and difficulty.
        let enemy = random_enemy_for_region_and_difficulty(
            &context.mission.region_id,
            context.mission.combat_difficulty(),
        );

        Self {
            players,
            current_player_idx: 0,
            enemy,
            hand,
            return_mission: Some(context),
            damage_taken: vec![0; party_size],
            stress_gained: vec![0; party_size],
            ..Default::default()
        }
    }

    pub fn update(&mut self) -> Option<StateTransition> {
        self.tick_feedback();

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
                self.select_card(i);
            }

            // Mouse click on card
            let (card_x, card_y, card_width, card_height) = combat_card_rect(i, self.hand.len());
            if was_pressed(card_x, card_y, card_width, card_height) {
                if self.selected_card == Some(i) {
                    // Clicking already selected card = play it
                    self.try_play_selected_card();
                } else {
                    self.select_card(i);
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
        let end_btn_x = screen_width() - 168.0;
        let end_btn_y = screen_height() - 58.0;
        if was_pressed(end_btn_x, end_btn_y, 144.0, 38.0) {
            self.end_turn();
        }

        // Check win/lose
        if self.enemy.hp <= 0 {
            // Victory - return to mission if we came from one
            if let Some(ctx) = &self.return_mission {
                // Update party member states with current HP/stress
                let updated_members: Vec<PartyMemberState> = self
                    .players
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let orig = ctx.party_members.get(i);
                        PartyMemberState {
                            id: orig.map(|m| m.id.clone()).unwrap_or_default(),
                            name: p.name.clone(),
                            hp: p.hp,
                            max_hp: p.max_hp,
                            stress: p.stress,
                            image_path: p.image_path.clone(),
                            class_name: orig
                                .map(|m| m.class_name.clone())
                                .unwrap_or_else(|| "Soldier".to_string()),
                            deck_additions: orig
                                .map(|m| m.deck_additions.clone())
                                .unwrap_or_default(),
                            traumas: p.traumas.clone(),
                            resolve_state: p.resolve_state.clone(),
                        }
                    })
                    .collect();

                let mission_state =
                    MissionState::from_mission_with_party(ctx.mission.clone(), updated_members)
                        .with_node(ctx.current_node)
                        .with_map_nodes(ctx.map_nodes.clone())
                        .with_visited(ctx.visited_nodes.clone());
                return Some(StateTransition::ToMission(mission_state));
            } else {
                // Not from mission - just show simple victory
                let leader_id = self.players.first().map(|p| p.name.as_str()).unwrap_or("");
                return Some(StateTransition::ToResults(ResultState::victory_for(
                    leader_id,
                )));
            }
        }

        // Check if all players are dead
        let all_dead = self.players.iter().all(|p| p.hp <= 0);
        if all_dead {
            // Defeat - always go to results
            if let Some(ctx) = &self.return_mission {
                let final_members = self.party_members_from_players(ctx);
                let results = ResultState::defeat_for_mission(&ctx.mission, &final_members);
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
        let Some(card_idx) = self.selected_card else {
            self.set_feedback("Select a card first.".to_string());
            return;
        };
        if card_idx >= self.hand.len() || self.current_player_idx >= self.players.len() {
            self.selected_card = None;
            self.set_feedback("That card is no longer available.".to_string());
            return;
        }

        let card = self.hand[card_idx].clone();
        let effective_cost = self.effective_card_cost(&card);
        let can_afford = effective_cost <= self.energy;
        let attack_blocked = card.is_attack() && self.resolver.turn_mods.attacks_disabled;

        if !can_afford {
            self.set_feedback(format!(
                "{} needs {} energy. You have {}.",
                card.name, effective_cost, self.energy
            ));
            return;
        }
        if attack_blocked {
            self.set_feedback(format!("{} is blocked this turn.", card.name));
            return;
        }

        if self.fearful_fumble(&card) {
            self.selected_card = None;
            self.set_feedback(format!("{} fumbled.", card.name));
            return;
        }

        let player_name = self.players[self.current_player_idx].name.clone();
        let card_name = card.name.clone();
        let effects = card.effects.clone();
        self.energy -= effective_cost;
        self.resolver
            .log
            .push(format!("{} plays {}", player_name, card_name));

        let player = &mut self.players[self.current_player_idx];
        for effect in effects {
            self.resolver.resolve(&effect, player, &mut self.enemy);
        }

        self.apply_card_turn_modifiers();
        self.hand.remove(card_idx);
        self.selected_card = None;
        self.set_feedback(format!("{} played.", card_name));
    }

    fn select_card(&mut self, idx: usize) {
        if let Some(card) = self.hand.get(idx) {
            self.selected_card = Some(idx);
            self.set_feedback(format!(
                "{} selected. Click again or press Enter to play.",
                card.name
            ));
        }
    }

    fn tick_feedback(&mut self) {
        if let Some((_, time)) = &mut self.feedback {
            *time -= get_frame_time();
            if *time <= 0.0 {
                self.feedback = None;
            }
        }
    }

    fn set_feedback(&mut self, text: String) {
        self.feedback = Some((text, 2.0));
    }

    fn effective_card_cost(&self, card: &Card) -> i32 {
        let Some(player) = self.players.get(self.current_player_idx) else {
            return card.cost;
        };

        let mut cost = card.cost;
        if player
            .traumas
            .iter()
            .any(|t| t.trauma_type == TraumaType::Broken)
        {
            cost += 1;
        }
        if card
            .effects
            .iter()
            .any(|e| matches!(e, crate::combat::CardEffect::Block(_)))
            && player
                .traumas
                .iter()
                .any(|t| t.trauma_type == TraumaType::Paranoid)
        {
            cost += 1;
        }
        cost
    }

    fn fearful_fumble(&mut self, card: &Card) -> bool {
        if !card.is_attack() {
            return false;
        }
        let Some(player) = self.players.get_mut(self.current_player_idx) else {
            return false;
        };
        if player
            .traumas
            .iter()
            .any(|t| t.trauma_type == TraumaType::Fearful)
            && macroquad_toolkit::rng::chance(0.15)
        {
            player.add_stress(3);
            self.resolver
                .log
                .push(format!("{} hesitates and loses the attack", player.name));
            true
        } else {
            false
        }
    }

    fn apply_card_turn_modifiers(&mut self) {
        if self.resolver.turn_mods.energy_to_gain > 0 {
            self.energy += self.resolver.turn_mods.energy_to_gain;
            self.resolver.turn_mods.energy_to_gain = 0;
        }

        if self.resolver.turn_mods.cards_to_draw > 0 {
            let count = self.resolver.turn_mods.cards_to_draw;
            self.draw_extra_cards(count);
            self.resolver.turn_mods.cards_to_draw = 0;
        }
    }

    fn draw_extra_cards(&mut self, count: i32) {
        let deck = self.deck_for_current_player();
        let mut remaining = count;
        for card in deck {
            if self.hand.len() >= 7 || remaining <= 0 {
                break;
            }
            if !self.hand.iter().any(|c| c.id == card.id) {
                self.hand.push(card);
                remaining -= 1;
            }
        }
    }

    fn deck_for_current_player(&self) -> Vec<Card> {
        let (class_name, deck_additions) = self
            .return_mission
            .as_ref()
            .and_then(|ctx| ctx.party_members.get(self.current_player_idx))
            .map(|m| (m.class_name.as_str(), m.deck_additions.as_slice()))
            .unwrap_or(("Soldier", &[]));
        Card::load_deck_for_class(class_name, deck_additions)
    }

    fn party_members_from_players(&self, ctx: &MissionContext) -> Vec<PartyMemberState> {
        self.players
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let orig = ctx.party_members.get(i);
                PartyMemberState {
                    id: orig.map(|m| m.id.clone()).unwrap_or_default(),
                    name: p.name.clone(),
                    hp: p.hp,
                    max_hp: p.max_hp,
                    stress: p.stress,
                    image_path: p.image_path.clone(),
                    class_name: orig
                        .map(|m| m.class_name.clone())
                        .unwrap_or_else(|| "Soldier".to_string()),
                    deck_additions: orig.map(|m| m.deck_additions.clone()).unwrap_or_default(),
                    traumas: p.traumas.clone(),
                    resolve_state: p.resolve_state.clone(),
                }
            })
            .collect()
    }

    fn end_turn(&mut self) {
        let actor_name = self
            .players
            .get(self.current_player_idx)
            .map(|player| player.name.clone())
            .unwrap_or_else(|| "Adventurer".to_string());
        let old_intent = self.enemy.intent.description();

        // Current player status tick and block reset
        if let Some(player) = self.players.get_mut(self.current_player_idx) {
            player.tick_statuses();
            player.block = 0;
        }

        // Enemy Action
        let (dmg, stress) = self.enemy.execute_intent();
        let enemy_acted = dmg > 0 || stress > 0;

        // Apply damage to current player
        let mut actual_damage = 0;
        if dmg > 0 {
            if let Some(player) = self.players.get_mut(self.current_player_idx) {
                let actual = player.take_damage(dmg);
                actual_damage = actual;
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
                if self.players[self.current_player_idx].hp > 0
                    || self.current_player_idx == start_idx
                {
                    break;
                }
            }
        }

        // Next Turn
        self.turn += 1;
        self.energy = self.max_energy + self.resolver.turn_mods.start_turn();

        // Draw class-appropriate cards for current player
        self.hand = self.deck_for_current_player().into_iter().take(5).collect();

        // Roll new enemy intent for next turn
        self.enemy.roll_intent(self.turn);

        self.resolver.log.push(format!(
            "End turn: {} resolved. {} took {} damage and {} stress.",
            old_intent, actor_name, actual_damage, base_stress
        ));
        self.resolver.log.push(format!(
            "Turn {} begins. Enemy intent: {}.",
            self.turn,
            self.enemy.intent.description()
        ));
        self.set_feedback(format!("Turn {} begins.", self.turn));
    }

    pub fn draw(&self, textures: &std::collections::HashMap<String, Texture2D>) {
        let region_id = if let Some(ctx) = &self.return_mission {
            &ctx.mission.region_id
        } else {
            "dark_woods"
        };

        let bg_path = format!("assets/images/regions/{}.png", region_id);
        if let Some(tex) = textures.get(&bg_path) {
            draw_texture_ex(
                tex,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        } else {
            clear_background(Color::from_rgba(9, 7, 6, 255));
        }
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 178),
        );

        draw_header(self.turn);
        draw_party_panel(
            &self.players,
            self.current_player_idx,
            self.energy,
            self.max_energy,
            textures,
        );
        draw_enemy_stage(&self.enemy, textures);

        let preview_idx = hovered_card_index(&self.hand).or(self.selected_card);
        draw_report_panel(self, preview_idx);
        draw_feedback_panel(self.feedback.as_ref());

        let mut hovered_card_idx: Option<usize> = None;
        for (i, card) in self.hand.iter().enumerate() {
            let (x, y, w, h) = combat_card_rect(i, self.hand.len());
            let is_hovered = crate::ui::is_mouse_over(x, y, w, h);
            if is_hovered {
                hovered_card_idx = Some(i);
            }
            let effective_cost = self.effective_card_cost(card);
            let can_afford = effective_cost <= self.energy;
            let attack_blocked = card.is_attack() && self.resolver.turn_mods.attacks_disabled;
            draw_combat_card(
                i,
                card,
                self.hand.len(),
                self.selected_card == Some(i),
                is_hovered,
                can_afford && !attack_blocked,
                attack_blocked,
                effective_cost,
                textures,
            );
        }

        let end_btn_x = screen_width() - 168.0;
        let end_btn_y = screen_height() - 58.0;
        draw_action_button("End Turn", end_btn_x, end_btn_y, 144.0, 38.0);
        draw_text(
            "Shortcuts: 1-5 Select - Enter Play - E End Turn",
            24.0,
            screen_height() - 26.0,
            14.0,
            muted_text_color(),
        );

        if let Some(idx) = hovered_card_idx {
            if let Some(card) = self.hand.get(idx) {
                crate::ui::card_tooltip(&card.name, &card.description);
            }
        }
    }
}

fn draw_header(turn: usize) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        72.0,
        Color::from_rgba(8, 7, 6, 232),
    );
    draw_line(0.0, 72.0, screen_width(), 72.0, 2.0, border_color());
    draw_text("COMBAT", 24.0, 42.0, 34.0, title_color());
    draw_text(&format!("Turn {}", turn), 188.0, 42.0, 20.0, candle_color());
}

fn draw_party_panel(
    players: &[Unit],
    current_player_idx: usize,
    energy: i32,
    max_energy: i32,
    textures: &std::collections::HashMap<String, Texture2D>,
) {
    panel(24.0, 92.0, 260.0, 328.0, "PLAYER AREA");
    let Some(active) = players.get(current_player_idx) else {
        return;
    };

    draw_text(&active.name, 44.0, 148.0, 24.0, title_color());
    draw_text(
        &format!(
            "HP {}/{}    Block {}",
            active.hp, active.max_hp, active.block
        ),
        44.0,
        178.0,
        16.0,
        text_color(),
    );
    draw_text(
        &format!(
            "Stress {}    Energy {}/{}",
            active.stress, energy, max_energy
        ),
        44.0,
        204.0,
        16.0,
        muted_text_color(),
    );
    if let Some(resolve) = &active.resolve_state {
        let (label, color) = match resolve {
            ResolveState::Virtuous => ("Virtuous", ready_color()),
            ResolveState::Afflicted => ("Afflicted", danger_color()),
        };
        draw_text(label, 44.0, 230.0, 16.0, color);
    }

    draw_text("Party", 44.0, 268.0, 16.0, candle_color());
    for (i, player) in players.iter().enumerate().take(4) {
        let y = 300.0 + (i as f32 * 30.0);
        let marker = if i == current_player_idx { ">" } else { " " };
        draw_text(marker, 44.0, y, 15.0, candle_color());
        if let Some(path) = &player.image_path {
            if let Some(tex) = textures.get(path) {
                draw_texture_ex(
                    tex,
                    64.0,
                    y - 20.0,
                    if player.hp <= 0 {
                        Color::from_rgba(90, 90, 90, 255)
                    } else {
                        WHITE
                    },
                    DrawTextureParams {
                        dest_size: Some(vec2(22.0, 22.0)),
                        ..Default::default()
                    },
                );
            }
        }
        draw_text(
            &format!("{}  {}/{}", player.name, player.hp, player.max_hp),
            94.0,
            y,
            14.0,
            if player.hp <= 0 {
                danger_color()
            } else {
                text_color()
            },
        );
    }
}

fn draw_enemy_stage(enemy: &Unit, textures: &std::collections::HashMap<String, Texture2D>) {
    panel(308.0, 92.0, 644.0, 204.0, "ENEMY AREA");
    let center_x = 630.0;

    if let Some(path) = &enemy.image_path {
        if let Some(tex) = textures.get(path) {
            draw_texture_ex(
                tex,
                center_x - 68.0,
                128.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(136.0, 136.0)),
                    ..Default::default()
                },
            );
        }
    } else {
        draw_circle(center_x, 190.0, 54.0, Color::from_rgba(62, 44, 38, 255));
    }

    let name_w = measure_text(&enemy.name, None, 26, 1.0).width;
    draw_text(
        &enemy.name,
        center_x - name_w / 2.0,
        128.0,
        26.0,
        title_color(),
    );
    let hp = format!("HP {}/{}    Block {}", enemy.hp, enemy.max_hp, enemy.block);
    let hp_w = measure_text(&hp, None, 17, 1.0).width;
    draw_text(&hp, center_x - hp_w / 2.0, 280.0, 17.0, text_color());

    let intent = format!("Intent: {}", enemy.intent.description());
    let intent_color = match &enemy.intent {
        crate::combat::EnemyIntent::Attack(_) => danger_color(),
        crate::combat::EnemyIntent::Block(_) => info_color(),
        crate::combat::EnemyIntent::Buff => candle_color(),
        crate::combat::EnemyIntent::Debuff => mystery_color(),
        crate::combat::EnemyIntent::Unknown => muted_text_color(),
    };
    draw_rectangle(720.0, 144.0, 196.0, 86.0, Color::from_rgba(22, 18, 16, 220));
    draw_rectangle_lines(720.0, 144.0, 196.0, 86.0, 1.0, intent_color);
    draw_text("NEXT", 740.0, 172.0, 16.0, muted_text_color());
    draw_wrapped_text(&intent, 740.0, 202.0, 156.0, 20.0, intent_color);

    if !enemy.statuses.is_empty() {
        let mut x = 332.0;
        for status in enemy.statuses.iter().take(4) {
            draw_text(
                &format!("{:?} {}", status.effect_type, status.duration),
                x,
                274.0,
                14.0,
                ready_color(),
            );
            x += 112.0;
        }
    }
}

fn draw_report_panel(state: &CombatState, preview_idx: Option<usize>) {
    panel(308.0, 314.0, 644.0, 106.0, "BATTLE REPORT");
    if let Some(idx) = preview_idx {
        if let Some(card) = state.hand.get(idx) {
            draw_text(&card.name, 330.0, 362.0, 20.0, candle_color());
            draw_wrapped_text(
                &card_preview(state, card),
                330.0,
                390.0,
                590.0,
                15.0,
                text_color(),
            );
            return;
        }
    }

    let mut y = 358.0;
    let mut drew_any = false;
    for line in state.resolver.log.iter().rev().take(3).rev() {
        draw_text(line, 330.0, y, 15.0, muted_text_color());
        y += 22.0;
        drew_any = true;
    }
    if !drew_any {
        let Some(player) = state.players.get(state.current_player_idx) else {
            return;
        };
        draw_text(
            &intent_warning(&player.name, &state.enemy.intent),
            330.0,
            372.0,
            16.0,
            muted_text_color(),
        );
    }
}

fn draw_combat_card(
    i: usize,
    card: &Card,
    hand_len: usize,
    selected: bool,
    hovered: bool,
    can_play: bool,
    attack_blocked: bool,
    effective_cost: i32,
    textures: &std::collections::HashMap<String, Texture2D>,
) {
    let (x, y, w, h) = combat_card_rect(i, hand_len);
    let accent = card_accent(card);
    let border = if selected {
        candle_color()
    } else if hovered && can_play {
        ready_color()
    } else if !can_play {
        danger_color()
    } else {
        accent
    };

    draw_rectangle(x, y, w, h, Color::from_rgba(18, 16, 14, 245));
    draw_rectangle_lines(x, y, w, h, if selected { 3.0 } else { 2.0 }, border);
    draw_rectangle(
        x + 6.0,
        y + 6.0,
        w - 12.0,
        26.0,
        Color::from_rgba(38, 32, 26, 246),
    );
    draw_text(
        &effective_cost.to_string(),
        x + 14.0,
        y + 25.0,
        20.0,
        if can_play {
            candle_color()
        } else {
            danger_color()
        },
    );
    draw_text(
        card_type(card),
        x + w - 64.0,
        y + 24.0,
        13.0,
        muted_text_color(),
    );

    let art_x = x + 8.0;
    let art_y = y + 38.0;
    let art_w = w - 16.0;
    let art_h = 78.0;
    draw_rectangle(
        art_x,
        art_y,
        art_w,
        art_h,
        Color::from_rgba(42, 38, 34, 255),
    );
    if let Some(path) = &card.image_path {
        if let Some(tex) = textures.get(path) {
            draw_texture_ex(
                tex,
                art_x,
                art_y,
                if attack_blocked {
                    Color::from_rgba(130, 105, 140, 255)
                } else {
                    WHITE
                },
                DrawTextureParams {
                    dest_size: Some(vec2(art_w, art_h)),
                    ..Default::default()
                },
            );
        }
    }
    draw_rectangle(art_x, art_y, art_w, art_h, Color::from_rgba(0, 0, 0, 55));

    draw_text(&card.name, x + 10.0, y + 136.0, 15.0, text_color());
    let status = if attack_blocked {
        "Blocked this turn"
    } else if can_play {
        "Ready"
    } else {
        "Need energy"
    };
    draw_text(
        status,
        x + 10.0,
        y + 158.0,
        12.0,
        if can_play {
            ready_color()
        } else {
            danger_color()
        },
    );
    draw_wrapped_text(
        &card.description,
        x + 10.0,
        y + 178.0,
        w - 20.0,
        11.0,
        muted_text_color(),
    );
}

fn draw_action_button(label: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(70, 49, 27, 238));
    draw_rectangle_lines(x, y, w, h, 1.0, candle_color());
    let tw = measure_text(label, None, 16, 1.0).width;
    draw_text(label, x + (w - tw) / 2.0, y + 24.0, 16.0, text_color());
}

fn panel(x: f32, y: f32, w: f32, h: f32, title: &str) {
    draw_rectangle(x, y, w, h, Color::from_rgba(13, 11, 10, 210));
    draw_rectangle(x, y, w, 32.0, Color::from_rgba(42, 30, 18, 222));
    draw_rectangle_lines(x, y, w, h, 1.0, border_color());
    draw_text(title, x + 14.0, y + 22.0, 15.0, candle_color());
}

fn combat_card_rect(i: usize, hand_len: usize) -> (f32, f32, f32, f32) {
    let card_w = 142.0;
    let card_h = 202.0;
    let gap = 14.0;
    let count = hand_len.max(1).min(5) as f32;
    let total_w = count * card_w + (count - 1.0) * gap;
    let x = (screen_width() - total_w) / 2.0 + (i as f32 * (card_w + gap));
    (x, screen_height() - 244.0, card_w, card_h)
}

fn hovered_card_index(hand: &[Card]) -> Option<usize> {
    for i in 0..hand.len().min(5) {
        let (x, y, w, h) = combat_card_rect(i, hand.len());
        if crate::ui::is_mouse_over(x, y, w, h) {
            return Some(i);
        }
    }
    None
}

fn card_preview(state: &CombatState, card: &Card) -> String {
    let player = state.players.get(state.current_player_idx);
    let mut parts = Vec::new();
    for effect in &card.effects {
        match effect {
            crate::combat::CardEffect::Damage(amount) => parts.push(format!(
                "Deal {} damage to {}. Enemy HP after: {}/{}.",
                amount,
                state.enemy.name,
                (state.enemy.hp - amount).max(0),
                state.enemy.max_hp
            )),
            crate::combat::CardEffect::Block(amount) => parts.push(format!(
                "Gain {} Block. Block after: {}.",
                amount,
                player.map(|p| p.block + amount).unwrap_or(*amount)
            )),
            crate::combat::CardEffect::Heal(amount) => parts.push(format!(
                "Heal {} HP. HP after: {}/{}.",
                amount,
                player
                    .map(|p| (p.hp + amount).min(p.max_hp))
                    .unwrap_or(*amount),
                player.map(|p| p.max_hp).unwrap_or(*amount)
            )),
            crate::combat::CardEffect::ReduceStress(amount) => {
                parts.push(format!("Reduce stress by {}.", amount));
            }
            crate::combat::CardEffect::DrawCards(amount) => {
                parts.push(format!("Draw {} card(s).", amount));
            }
            crate::combat::CardEffect::GainEnergy(amount) => {
                parts.push(format!("Gain {} energy this turn.", amount));
            }
            crate::combat::CardEffect::EnemyStress(amount) => {
                parts.push(format!("Apply {} stress to the enemy.", amount));
            }
            crate::combat::CardEffect::ApplyStatus {
                effect_type,
                duration,
                ..
            } => parts.push(format!("Apply {:?} for {} turn(s).", effect_type, duration)),
            _ => parts.push(card.description.clone()),
        }
    }
    if parts.is_empty() {
        card.description.clone()
    } else {
        parts.join(" ")
    }
}

fn intent_warning(player_name: &str, intent: &crate::combat::EnemyIntent) -> String {
    match intent {
        crate::combat::EnemyIntent::Attack(amount) => {
            format!(
                "{} will take {} damage unless blocked.",
                player_name, amount
            )
        }
        crate::combat::EnemyIntent::Block(amount) => {
            format!("{} will gain {} Block if left alone.", "Enemy", amount)
        }
        crate::combat::EnemyIntent::Buff => "Enemy is preparing a buff.".to_string(),
        crate::combat::EnemyIntent::Debuff => {
            format!("{} is about to be weakened or stressed.", player_name)
        }
        crate::combat::EnemyIntent::Unknown => "Enemy intent is hidden.".to_string(),
    }
}

fn card_type(card: &Card) -> &'static str {
    if card.is_attack() {
        "Attack"
    } else if card
        .effects
        .iter()
        .any(|effect| matches!(effect, crate::combat::CardEffect::Block(_)))
    {
        "Guard"
    } else if card
        .effects
        .iter()
        .any(|effect| matches!(effect, crate::combat::CardEffect::Heal(_)))
    {
        "Heal"
    } else if card.effects.iter().any(|effect| {
        matches!(
            effect,
            crate::combat::CardEffect::EnemyStress(_)
                | crate::combat::CardEffect::ApplyStatus { .. }
        )
    }) {
        "Mystic"
    } else {
        "Skill"
    }
}

fn card_accent(card: &Card) -> Color {
    match card_type(card) {
        "Attack" => Color::from_rgba(143, 61, 49, 255),
        "Guard" => Color::from_rgba(105, 128, 139, 255),
        "Heal" => Color::from_rgba(128, 160, 96, 255),
        "Mystic" => Color::from_rgba(132, 96, 158, 255),
        _ => Color::from_rgba(171, 126, 62, 255),
    }
}

fn draw_wrapped_text(text: &str, x: f32, y: f32, max_width: f32, font_size: f32, color: Color) {
    let mut line = String::new();
    let mut line_y = y;
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };
        if measure_text(&candidate, None, font_size as u16, 1.0).width > max_width
            && !line.is_empty()
        {
            draw_text(&line, x, line_y, font_size, color);
            line = word.to_string();
            line_y += font_size + 5.0;
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        draw_text(&line, x, line_y, font_size, color);
    }
}

fn text_color() -> Color {
    Color::from_rgba(230, 221, 205, 255)
}

fn muted_text_color() -> Color {
    Color::from_rgba(158, 145, 126, 255)
}

fn title_color() -> Color {
    Color::from_rgba(239, 224, 190, 255)
}

fn candle_color() -> Color {
    Color::from_rgba(207, 151, 54, 255)
}

fn ready_color() -> Color {
    Color::from_rgba(130, 177, 101, 255)
}

fn danger_color() -> Color {
    Color::from_rgba(168, 58, 48, 255)
}

fn info_color() -> Color {
    Color::from_rgba(118, 151, 164, 255)
}

fn mystery_color() -> Color {
    Color::from_rgba(138, 104, 167, 255)
}

fn border_color() -> Color {
    Color::from_rgba(105, 76, 43, 210)
}
