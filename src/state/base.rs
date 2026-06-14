//! Kingdom base state - command-table management UI.

use super::{MissionSelectState, StateTransition};
use crate::kingdom::{Adventurer, Building, KingdomState, Party, Roster};
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

const UI_BG_PATH: &str = "assets/images/ui/command_table.png";
const HEADER_H: f32 = 92.0;
const MAIN_Y: f32 = 110.0;
const MAIN_H: f32 = 245.0;
const ACTION_Y: f32 = 372.0;
const ACTION_H: f32 = 76.0;
const DETAIL_Y: f32 = 466.0;
const SIDE_PAD: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseTab {
    Kingdom,
    Roster,
    Missions,
    Buildings,
    DeckTraining,
    Graveyard,
    Journal,
}

impl BaseTab {
    const ALL: [BaseTab; 7] = [
        BaseTab::Kingdom,
        BaseTab::Roster,
        BaseTab::Missions,
        BaseTab::Buildings,
        BaseTab::DeckTraining,
        BaseTab::Graveyard,
        BaseTab::Journal,
    ];

    fn label(self) -> &'static str {
        match self {
            BaseTab::Kingdom => "Kingdom",
            BaseTab::Roster => "Roster",
            BaseTab::Missions => "Missions",
            BaseTab::Buildings => "Buildings",
            BaseTab::DeckTraining => "Deck / Training",
            BaseTab::Graveyard => "Graveyard",
            BaseTab::Journal => "Journal",
        }
    }

    fn next(self) -> Self {
        let idx = Self::ALL.iter().position(|tab| *tab == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }
}

/// Focus area for compatibility with party formation flow.
#[derive(PartialEq, Clone)]
pub enum FocusArea {
    Roster,
    Buildings,
    PartyFormation,
}

impl Default for FocusArea {
    fn default() -> Self {
        FocusArea::Roster
    }
}

/// State for managing the kingdom base.
pub struct BaseState {
    pub selected_building: Option<usize>,
    pub selected_adventurer: Option<usize>,
    pub focus: FocusArea,
    pub active_tab: BaseTab,
    pub viewing_deck: bool,
    /// Current party being formed.
    pub forming_party: Party,
}

impl Default for BaseState {
    fn default() -> Self {
        Self {
            selected_building: Some(0),
            selected_adventurer: Some(0),
            focus: FocusArea::Roster,
            active_tab: BaseTab::Kingdom,
            viewing_deck: false,
            forming_party: Party::default(),
        }
    }
}

impl BaseState {
    pub fn update(
        &mut self,
        kingdom: &mut KingdomState,
        roster: &mut Roster,
    ) -> Option<StateTransition> {
        if self.viewing_deck {
            if is_key_pressed(KeyCode::Escape) {
                self.viewing_deck = false;
            }
            return None;
        }

        if self.focus == FocusArea::PartyFormation {
            return self.update_party_formation(roster);
        }

        if is_key_pressed(KeyCode::Tab) {
            self.active_tab = self.active_tab.next();
            self.focus = if self.active_tab == BaseTab::Buildings {
                FocusArea::Buildings
            } else {
                FocusArea::Roster
            };
        }

        self.update_tabs();
        if let Some(transition) = self.update_action_buttons(kingdom, roster) {
            return Some(transition);
        }
        if self.update_detail_buttons() {
            return None;
        }
        self.update_selection(kingdom, roster);
        self.update_shortcuts(kingdom, roster)
    }

    fn update_party_formation(&mut self, roster: &Roster) -> Option<StateTransition> {
        for i in 0..roster.adventurers.len().min(9) {
            let key = number_key(i);
            let y = MAIN_Y + 42.0 + (i as f32 * 40.0);
            let clicked = crate::ui::was_clicked(44.0, y - 24.0, 720.0, 34.0);

            if key.is_some_and(is_key_pressed) || clicked {
                if let Some(adv) = roster.adventurers.get(i) {
                    if self.forming_party.contains(&adv.id) {
                        if self.forming_party.leader_id() != Some(adv.id.as_str()) {
                            self.forming_party.remove_member(&adv.id);
                        }
                    } else if !self.forming_party.is_full() {
                        self.forming_party.add_member(&adv.id);
                    }
                }
            }
        }

        let (mission_x, mission_y, mission_w, mission_h) = party_mission_button_rect();
        if crate::ui::was_clicked(mission_x, mission_y, mission_w, mission_h)
            && !self.forming_party.is_empty()
        {
            return Some(StateTransition::ToMissionSelect(
                MissionSelectState::for_party(self.forming_party.clone(), roster),
            ));
        }

        let (back_x, back_y, back_w, back_h) = party_back_button_rect();
        if crate::ui::was_clicked(back_x, back_y, back_w, back_h) {
            self.forming_party = Party::default();
            self.focus = FocusArea::Roster;
            self.active_tab = BaseTab::Roster;
            return None;
        }

        if is_key_pressed(KeyCode::Enter) && !self.forming_party.is_empty() {
            return Some(StateTransition::ToMissionSelect(
                MissionSelectState::for_party(self.forming_party.clone(), roster),
            ));
        }

        if is_key_pressed(KeyCode::Escape) {
            self.forming_party = Party::default();
            self.focus = FocusArea::Roster;
            self.active_tab = BaseTab::Roster;
        }

        None
    }

    fn update_tabs(&mut self) {
        let mut x = SIDE_PAD;
        for tab in BaseTab::ALL {
            let w = tab_width(tab);
            if crate::ui::was_clicked(x, 62.0, w, 28.0) {
                self.active_tab = tab;
                self.focus = if tab == BaseTab::Buildings {
                    FocusArea::Buildings
                } else {
                    FocusArea::Roster
                };
            }
            x += w + 8.0;
        }
    }

    fn update_action_buttons(
        &mut self,
        kingdom: &mut KingdomState,
        roster: &mut Roster,
    ) -> Option<StateTransition> {
        for (i, action) in action_buttons().iter().enumerate() {
            let x = SIDE_PAD + 18.0 + (i as f32 * 138.0);
            if crate::ui::was_clicked(x, ACTION_Y + 30.0, 126.0, 30.0) {
                match *action {
                    "Embark" => self.start_party_from_selected(roster),
                    "Roster" => {
                        self.active_tab = BaseTab::Roster;
                        self.focus = FocusArea::Roster;
                    }
                    "Facilities" => {
                        self.active_tab = BaseTab::Buildings;
                        self.focus = FocusArea::Buildings;
                    }
                    "Treat" => self.treat_selected_adventurer(kingdom, roster),
                    "Recruit" => {
                        if kingdom.has_building("guild_hall") {
                            return Some(StateTransition::ToRecruit);
                        }
                    }
                    "Decks" => {
                        if self.selected_adventurer.is_some() {
                            self.viewing_deck = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn update_detail_buttons(&mut self) -> bool {
        if self.selected_adventurer.is_none() && self.selected_building.is_none() {
            return false;
        }

        let (x, y, w, h) = detail_back_button_rect();
        if crate::ui::was_clicked(x, y, w, h) {
            self.selected_adventurer = None;
            self.selected_building = None;
            self.viewing_deck = false;
            return true;
        }

        false
    }

    fn update_selection(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) {
        match self.active_tab {
            BaseTab::Buildings => {
                let count = kingdom.buildings.len().min(9);
                for i in 0..count {
                    let key = number_key(i);
                    let (x, y, w, h) = facility_card_rect(i);
                    if key.is_some_and(is_key_pressed) || crate::ui::was_clicked(x, y, w, h) {
                        if self.selected_building == Some(i)
                            && crate::ui::was_clicked(x, y, w, h)
                            && self.can_build(kingdom, i)
                        {
                            self.try_construct_building(kingdom, i);
                        } else {
                            self.selected_building = Some(i);
                            self.selected_adventurer = None;
                        }
                    }
                }
            }
            BaseTab::Graveyard | BaseTab::Journal => {}
            _ => {
                let count = roster.adventurers.len().min(9);
                for i in 0..count {
                    let key = number_key(i);
                    let (x, y, w, h) = adventurer_row_hit_rect(self.active_tab, i);
                    let clicked = matches!(self.active_tab, BaseTab::Kingdom | BaseTab::Roster)
                        && crate::ui::was_clicked(x, y, w, h);
                    if key.is_some_and(is_key_pressed) || clicked {
                        if self.selected_adventurer == Some(i) && clicked {
                            self.start_party_from_selected(roster);
                        } else {
                            self.selected_adventurer = Some(i);
                            self.selected_building = None;
                        }
                    }
                }
            }
        }
    }

    fn update_shortcuts(
        &mut self,
        kingdom: &mut KingdomState,
        roster: &mut Roster,
    ) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::M) {
            self.start_party_from_selected(roster);
        }

        if is_key_pressed(KeyCode::D) && self.selected_adventurer.is_some() {
            self.viewing_deck = true;
        }

        if is_key_pressed(KeyCode::H) || is_key_pressed(KeyCode::T) {
            self.treat_selected_adventurer(kingdom, roster);
        }

        if is_key_pressed(KeyCode::U) {
            if let Some(adv_idx) = self.selected_adventurer {
                self.try_unlock_card(kingdom, roster, adv_idx);
            }
        }

        if is_key_pressed(KeyCode::R) && kingdom.has_building("guild_hall") {
            return Some(StateTransition::ToRecruit);
        }

        if is_key_pressed(KeyCode::Enter) && self.active_tab == BaseTab::Buildings {
            if let Some(idx) = self.selected_building {
                self.try_construct_building(kingdom, idx);
            }
        }

        None
    }

    fn start_party_from_selected(&mut self, roster: &Roster) {
        let idx = self.selected_adventurer.unwrap_or(0);
        if let Some(adventurer) = roster.adventurers.get(idx) {
            self.forming_party = Party::with_leader(&adventurer.id);
            self.focus = FocusArea::PartyFormation;
            self.active_tab = BaseTab::Roster;
        }
    }

    fn treat_selected_adventurer(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) {
        let Some(idx) = self.selected_adventurer else {
            return;
        };
        let Some(adv) = roster.adventurers.get_mut(idx) else {
            return;
        };

        if kingdom.has_building("infirmary") && adv.hp < adv.max_hp && kingdom.stats.supplies >= 10
        {
            adv.heal(10);
            kingdom.stats.supplies -= 10;
            return;
        }

        if kingdom.has_building("chapel") && adv.stress > 0 && kingdom.stats.supplies >= 10 {
            adv.reduce_stress(20);
            kingdom.stats.supplies -= 10;
        }
    }

    fn can_build(&self, kingdom: &KingdomState, idx: usize) -> bool {
        kingdom.buildings.get(idx).is_some_and(|building| {
            !building.built
                && kingdom.stats.gold >= building.cost_gold
                && kingdom.stats.supplies >= building.cost_supplies
        })
    }

    /// Try to construct a building at the given index.
    fn try_construct_building(&mut self, kingdom: &mut KingdomState, idx: usize) {
        if !self.can_build(kingdom, idx) {
            return;
        }

        if let Some(building) = kingdom.buildings.get_mut(idx) {
            kingdom.stats.gold -= building.cost_gold;
            kingdom.stats.supplies -= building.cost_supplies;
            building.built = true;
            building.level = 1;
            if building.id == "citadel" {
                kingdom.game_won = true;
            }
        }
    }

    fn try_unlock_card(&mut self, kingdom: &mut KingdomState, roster: &mut Roster, adv_idx: usize) {
        if !kingdom.has_building("foundry") {
            return;
        }

        let Some(adv) = roster.adventurers.get(adv_idx) else {
            return;
        };

        let class_name = format!("{:?}", adv.class);
        let known_cards = adv.deck_additions.clone();
        let Ok(all_cards) = crate::data::cards::CardData::load_all() else {
            return;
        };

        let mut candidates: Vec<_> = all_cards
            .iter()
            .filter(|card| {
                card.class_matches(&class_name)
                    && card.is_unlockable()
                    && !known_cards.iter().any(|id| id == &card.id)
            })
            .collect();
        candidates.sort_by_key(|card| card.required_knowledge);

        if let Some(card) = candidates
            .into_iter()
            .find(|card| kingdom.stats.knowledge >= card.required_knowledge)
        {
            kingdom.stats.knowledge -= card.required_knowledge;
            if let Some(adv) = roster.adventurers.get_mut(adv_idx) {
                adv.deck_additions.push(card.id.clone());
            }
        }
    }

    pub fn draw(
        &self,
        kingdom: &KingdomState,
        roster: &Roster,
        textures: &std::collections::HashMap<String, Texture2D>,
    ) {
        draw_command_table_background(textures);
        draw_header(kingdom, self.active_tab);
        draw_tabs(self.active_tab);

        if self.focus == FocusArea::PartyFormation {
            self.draw_party_formation(roster);
        } else {
            match self.active_tab {
                BaseTab::Kingdom => self.draw_kingdom_dashboard(kingdom, roster),
                BaseTab::Roster => self.draw_roster_tab(kingdom, roster),
                BaseTab::Missions => self.draw_missions_tab(kingdom, roster),
                BaseTab::Buildings => self.draw_buildings_tab(kingdom),
                BaseTab::DeckTraining => self.draw_deck_training_tab(kingdom, roster),
                BaseTab::Graveyard => self.draw_graveyard_tab(roster),
                BaseTab::Journal => self.draw_journal_tab(kingdom),
            }

            self.draw_action_bar(kingdom, roster);
            self.draw_detail_panel(kingdom, roster);
        }

        if self.viewing_deck {
            self.draw_deck_overlay(roster);
        }

        draw_shortcuts();
    }

    fn draw_kingdom_dashboard(&self, kingdom: &KingdomState, roster: &Roster) {
        let w = screen_width();
        let left_w = 220.0;
        let center_w = (w - 96.0) * 0.48;
        let right_x = SIDE_PAD + left_w + center_w + 24.0;
        let right_w = (w - right_x - SIDE_PAD).max(260.0);

        draw_resources_panel(kingdom, SIDE_PAD, MAIN_Y, left_w, MAIN_H);
        draw_adventurer_summary(
            roster,
            self.selected_adventurer,
            SIDE_PAD + left_w + 12.0,
            MAIN_Y,
            center_w,
            MAIN_H,
            self.forming_party.leader_id(),
        );
        draw_goals_panel(kingdom, right_x, MAIN_Y, right_w, MAIN_H);
    }

    fn draw_roster_tab(&self, kingdom: &KingdomState, roster: &Roster) {
        panel(SIDE_PAD, MAIN_Y, 735.0, MAIN_H, "ADVENTURERS");
        for (i, adv) in roster.adventurers.iter().enumerate().take(5) {
            draw_adventurer_row(
                i,
                adv,
                self.selected_adventurer == Some(i),
                44.0,
                MAIN_Y + 52.0,
            );
        }

        panel(790.0, MAIN_Y, screen_width() - 814.0, MAIN_H, "READINESS");
        draw_readiness_summary(kingdom, roster, 812.0, MAIN_Y + 48.0);
    }

    fn draw_missions_tab(&self, kingdom: &KingdomState, roster: &Roster) {
        panel(SIDE_PAD, MAIN_Y, 520.0, MAIN_H, "MISSION BOARD");
        let mut y = MAIN_Y + 50.0;
        for (quest, done) in kingdom.quest_log() {
            let color = if done { ready_color() } else { text_color() };
            draw_ui_text(
                &format!("{} {}", if done { "[x]" } else { "[ ]" }, quest),
                48.0,
                y,
                18.0,
                color,
            );
            y += 32.0;
        }

        panel(565.0, MAIN_Y, screen_width() - 589.0, MAIN_H, "EMBARK PREP");
        let selected = self
            .selected_adventurer
            .and_then(|idx| roster.adventurers.get(idx))
            .or_else(|| roster.adventurers.first());
        if let Some(adv) = selected {
            draw_ui_text(
                "Selected Leader",
                592.0,
                MAIN_Y + 50.0,
                18.0,
                muted_text_color(),
            );
            draw_ui_text(&adv.name, 592.0, MAIN_Y + 82.0, 26.0, candle_color());
            draw_ui_text(
                &format!(
                    "{} - HP {}/{} - Stress {} - {}",
                    format!("{:?}", adv.class),
                    adv.hp,
                    adv.max_hp,
                    adv.stress,
                    readiness_label(adv)
                ),
                592.0,
                MAIN_Y + 114.0,
                18.0,
                text_color(),
            );
            draw_wrapped_text(
                "Embark opens the mission board with this hero as party leader. Add more exhausted hands only if the route demands it.",
                592.0,
                MAIN_Y + 150.0,
                screen_width() - 630.0,
                16.0,
                muted_text_color(),
            );
        }
    }

    fn draw_buildings_tab(&self, kingdom: &KingdomState) {
        panel(
            SIDE_PAD,
            MAIN_Y,
            screen_width() - SIDE_PAD * 2.0,
            MAIN_H,
            "FACILITIES",
        );
        for (i, building) in kingdom.buildings.iter().enumerate() {
            draw_facility_card(
                i,
                building,
                self.selected_building == Some(i),
                self.can_build(kingdom, i),
            );
        }
    }

    fn draw_deck_training_tab(&self, kingdom: &KingdomState, roster: &Roster) {
        panel(SIDE_PAD, MAIN_Y, 430.0, MAIN_H, "TRAINING");
        let selected = self
            .selected_adventurer
            .and_then(|idx| roster.adventurers.get(idx));
        if let Some(adv) = selected {
            draw_ui_text(&adv.name, 48.0, MAIN_Y + 54.0, 24.0, candle_color());
            draw_ui_text(
                &format!("Deck: {} cards", deck_size(adv)),
                48.0,
                MAIN_Y + 86.0,
                18.0,
                text_color(),
            );
            let foundry_status = if kingdom.has_building("foundry") {
                "Foundry built. Press U to learn the next affordable card."
            } else {
                "Build the Foundry before advanced card training."
            };
            draw_wrapped_text(
                foundry_status,
                48.0,
                MAIN_Y + 120.0,
                360.0,
                16.0,
                muted_text_color(),
            );
        }

        panel(
            480.0,
            MAIN_Y,
            screen_width() - 504.0,
            MAIN_H,
            "TRAINING NOTES",
        );
        draw_wrapped_text(
            "Training is intentionally tied to Knowledge. The command table should make upgrades feel like hard choices, not automatic shopping.",
            506.0,
            MAIN_Y + 55.0,
            screen_width() - 555.0,
            18.0,
            text_color(),
        );
    }

    fn draw_graveyard_tab(&self, roster: &Roster) {
        panel(
            SIDE_PAD,
            MAIN_Y,
            screen_width() - SIDE_PAD * 2.0,
            MAIN_H,
            "GRAVEYARD / TRAUMA LOG",
        );
        if roster.graveyard.is_empty() {
            draw_ui_text(
                "No names carved into the boards yet.",
                48.0,
                MAIN_Y + 62.0,
                20.0,
                muted_text_color(),
            );
        } else {
            for (i, adv) in roster.graveyard.iter().enumerate().take(5) {
                draw_ui_text(
                    &adv.name,
                    48.0,
                    MAIN_Y + 58.0 + (i as f32 * 32.0),
                    20.0,
                    danger_color(),
                );
            }
        }
    }

    fn draw_journal_tab(&self, kingdom: &KingdomState) {
        panel(
            SIDE_PAD,
            MAIN_Y,
            screen_width() - SIDE_PAD * 2.0,
            MAIN_H,
            "JOURNAL",
        );
        let event = kingdom
            .last_event
            .as_deref()
            .unwrap_or("No fresh omens from the frontier.");
        draw_wrapped_text(
            event,
            48.0,
            MAIN_Y + 58.0,
            screen_width() - 96.0,
            18.0,
            text_color(),
        );
    }

    fn draw_party_formation(&self, roster: &Roster) {
        panel(
            SIDE_PAD,
            MAIN_Y,
            screen_width() - SIDE_PAD * 2.0,
            475.0,
            "PARTY / ADVENTURERS",
        );
        draw_ui_text(
            &format!(
                "Choose the expedition party ({}/{})",
                self.forming_party.size(),
                crate::kingdom::MAX_PARTY_SIZE
            ),
            48.0,
            MAIN_Y + 48.0,
            22.0,
            candle_color(),
        );

        for (i, adv) in roster.adventurers.iter().enumerate().take(9) {
            let y = MAIN_Y + 90.0 + (i as f32 * 40.0);
            let in_party = self.forming_party.contains(&adv.id);
            let leader = self.forming_party.leader_id() == Some(adv.id.as_str());
            let marker = if leader {
                "LEADER"
            } else if in_party {
                "ASSIGNED"
            } else {
                ""
            };
            let color = if in_party {
                candle_color()
            } else {
                text_color()
            };
            draw_ui_text(
                &format!(
                    "[{}] {:<18} {:<9} HP {}/{}  Stress {}  {}",
                    i + 1,
                    adv.name,
                    format!("{:?}", adv.class),
                    adv.hp,
                    adv.max_hp,
                    adv.stress,
                    marker
                ),
                48.0,
                y,
                18.0,
                color,
            );
        }

        draw_ui_text(
            "[Enter] Open Mission Board  [Esc] Cancel",
            48.0,
            screen_height() - 38.0,
            18.0,
            muted_text_color(),
        );

        let (mission_x, mission_y, mission_w, mission_h) = party_mission_button_rect();
        draw_action_button(
            "Open Mission Board",
            mission_x,
            mission_y,
            mission_w,
            mission_h,
            !self.forming_party.is_empty(),
        );
        let (back_x, back_y, back_w, back_h) = party_back_button_rect();
        draw_action_button("Back to Roster", back_x, back_y, back_w, back_h, true);
    }

    fn draw_action_bar(&self, kingdom: &KingdomState, roster: &Roster) {
        panel(
            SIDE_PAD,
            ACTION_Y,
            screen_width() - SIDE_PAD * 2.0,
            ACTION_H,
            "KINGDOM ACTIONS",
        );
        for (i, action) in action_buttons().iter().enumerate() {
            let x = SIDE_PAD + 18.0 + (i as f32 * 138.0);
            let enabled = action_enabled(action, kingdom, roster, self.selected_adventurer);
            draw_action_button(action, x, ACTION_Y + 30.0, 126.0, 30.0, enabled);
        }
    }

    fn draw_detail_panel(&self, kingdom: &KingdomState, roster: &Roster) {
        let h = (screen_height() - DETAIL_Y - 48.0).max(185.0);
        panel(
            SIDE_PAD,
            DETAIL_Y,
            screen_width() - SIDE_PAD * 2.0,
            h,
            "DETAILS",
        );
        if self.selected_adventurer.is_some() || self.selected_building.is_some() {
            let (x, y, w, h) = detail_back_button_rect();
            draw_action_button("Back", x, y, w, h, true);
        }

        match self.active_tab {
            BaseTab::Buildings => {
                if let Some(idx) = self.selected_building {
                    if let Some(building) = kingdom.buildings.get(idx) {
                        draw_building_details(building, kingdom, self.can_build(kingdom, idx));
                        return;
                    }
                }
            }
            BaseTab::Missions | BaseTab::Journal => {
                draw_goals_detail(kingdom);
                return;
            }
            BaseTab::Graveyard => {
                draw_graveyard_detail(roster);
                return;
            }
            BaseTab::DeckTraining => {
                if let Some(adv) = self
                    .selected_adventurer
                    .and_then(|idx| roster.adventurers.get(idx))
                {
                    draw_training_detail(adv, kingdom);
                    return;
                }
            }
            _ => {}
        }

        if let Some(adv) = self
            .selected_adventurer
            .and_then(|idx| roster.adventurers.get(idx))
        {
            draw_adventurer_details(adv);
        } else {
            draw_ui_text(
                "Select an adventurer, facility, or goal.",
                48.0,
                DETAIL_Y + 56.0,
                20.0,
                muted_text_color(),
            );
        }
    }

    fn draw_deck_overlay(&self, roster: &Roster) {
        let Some(adv) = self
            .selected_adventurer
            .and_then(|idx| roster.adventurers.get(idx))
        else {
            return;
        };

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(4, 3, 2, 235),
        );
        panel(
            34.0,
            36.0,
            screen_width() - 68.0,
            screen_height() - 72.0,
            "DECK / TRAINING",
        );
        draw_ui_text(
            &format!("{}'s Deck", adv.name),
            62.0,
            88.0,
            34.0,
            candle_color(),
        );
        draw_ui_text(
            "[Esc] Close",
            screen_width() - 170.0,
            86.0,
            18.0,
            muted_text_color(),
        );

        let class_name = format!("{:?}", adv.class);
        let deck = crate::combat::Card::load_deck_for_class(&class_name, &adv.deck_additions);
        let start_x = 62.0;
        let start_y = 122.0;
        let card_w = 132.0;
        let card_h = 164.0;
        let gap = 16.0;
        let cols = ((screen_width() - 124.0) / (card_w + gap)).max(1.0) as i32;

        for (i, card) in deck.iter().enumerate() {
            let row = (i as i32) / cols;
            let col = (i as i32) % cols;
            let x = start_x + (col as f32 * (card_w + gap));
            let y = start_y + (row as f32 * (card_h + gap));
            draw_card_frame(card, x, y, card_w, card_h, false);
        }
    }
}

fn draw_command_table_background(textures: &std::collections::HashMap<String, Texture2D>) {
    clear_background(Color::from_rgba(14, 10, 8, 255));
    if let Some(tex) = textures.get(UI_BG_PATH) {
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
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), table_color());
        draw_circle(190.0, 120.0, 86.0, Color::from_rgba(120, 70, 28, 50));
        draw_circle(
            screen_width() - 160.0,
            82.0,
            72.0,
            Color::from_rgba(150, 92, 34, 45),
        );
    }
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(8, 6, 5, 148),
    );
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(40, 24, 10, 68),
    );
}

fn draw_header(kingdom: &KingdomState, active_tab: BaseTab) {
    let morale = morale_label(kingdom.stats.morale);
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        HEADER_H,
        Color::from_rgba(10, 7, 6, 218),
    );
    draw_line(0.0, HEADER_H, screen_width(), HEADER_H, 2.0, border_color());
    draw_ui_text("FRONTIER KINGDOM", SIDE_PAD, 38.0, 34.0, title_color());
    draw_ui_text(
        &format!(
            "Day {}    Threat {}    Morale: {}    {}",
            kingdom.day,
            kingdom.threat_level,
            morale,
            active_tab.label()
        ),
        430.0,
        36.0,
        20.0,
        muted_text_color(),
    );
}

fn draw_tabs(active_tab: BaseTab) {
    let mut x = SIDE_PAD;
    for tab in BaseTab::ALL {
        let w = tab_width(tab);
        let selected = tab == active_tab;
        draw_rectangle(
            x,
            62.0,
            w,
            28.0,
            if selected {
                Color::from_rgba(97, 66, 27, 235)
            } else {
                Color::from_rgba(26, 23, 21, 210)
            },
        );
        draw_rectangle_lines(
            x,
            62.0,
            w,
            28.0,
            if selected { 2.0 } else { 1.0 },
            if selected {
                candle_color()
            } else {
                border_color()
            },
        );
        draw_ui_text(
            tab.label(),
            x + 10.0,
            82.0,
            16.0,
            if selected {
                title_color()
            } else {
                muted_text_color()
            },
        );
        x += w + 8.0;
    }
}

fn draw_resources_panel(kingdom: &KingdomState, x: f32, y: f32, w: f32, h: f32) {
    panel(x, y, w, h, "RESOURCES");
    let stats = &kingdom.stats;
    let rows = [
        ("Gold", stats.gold, candle_color()),
        ("Supplies", stats.supplies, parchment_color()),
        ("Security", stats.security, info_color()),
        ("Morale", stats.morale, morale_color(stats.morale)),
        ("Knowledge", stats.knowledge, mystery_color()),
        ("Influence", stats.influence, muted_text_color()),
    ];
    let mut row_y = y + 52.0;
    for (label, value, color) in rows {
        draw_ui_text(label, x + 18.0, row_y, 17.0, muted_text_color());
        draw_ui_text(&value.to_string(), x + w - 70.0, row_y, 20.0, color);
        row_y += 30.0;
    }
}

fn draw_adventurer_summary(
    roster: &Roster,
    selected: Option<usize>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    leader_id: Option<&str>,
) {
    panel(x, y, w, h, "PARTY / ADVENTURERS");
    for (i, adv) in roster.adventurers.iter().enumerate().take(5) {
        draw_adventurer_row(i, adv, selected == Some(i), x + 20.0, y + 50.0);
        if leader_id == Some(adv.id.as_str()) {
            draw_ui_text(
                "Leader",
                x + w - 90.0,
                y + 74.0 + (i as f32 * 34.0),
                15.0,
                candle_color(),
            );
        }
    }
}

fn draw_goals_panel(kingdom: &KingdomState, x: f32, y: f32, w: f32, h: f32) {
    panel(x, y, w, h, "CURRENT GOALS");
    let mut row_y = y + 48.0;
    for (quest, done) in kingdom.quest_log().into_iter().take(5) {
        let color = if done { ready_color() } else { text_color() };
        let marker = if done { "[x]" } else { "[ ]" };
        draw_ui_text(
            &format!("{} {}", marker, quest),
            x + 18.0,
            row_y,
            16.0,
            color,
        );
        row_y += 28.0;
    }
    if let Some(event) = &kingdom.last_event {
        draw_ui_text("Alert", x + 18.0, y + h - 58.0, 16.0, danger_color());
        draw_wrapped_text(
            event,
            x + 18.0,
            y + h - 35.0,
            w - 36.0,
            14.0,
            muted_text_color(),
        );
    }
}

fn draw_readiness_summary(kingdom: &KingdomState, roster: &Roster, x: f32, y: f32) {
    let ready = roster
        .adventurers
        .iter()
        .filter(|adv| readiness_label(adv) == "Ready")
        .count();
    draw_ui_text(
        &format!("Ready heroes: {}", ready),
        x,
        y,
        20.0,
        ready_color(),
    );
    draw_ui_text(
        &format!("Supplies available: {}", kingdom.stats.supplies),
        x,
        y + 34.0,
        18.0,
        parchment_color(),
    );
    draw_wrapped_text(
        "Send ready heroes, rest the shaken, and keep enough supplies for treatment after the mission.",
        x,
        y + 72.0,
        screen_width() - x - 56.0,
        16.0,
        muted_text_color(),
    );
}

fn draw_adventurer_row(i: usize, adv: &Adventurer, selected: bool, x: f32, start_y: f32) {
    let y = start_y + (i as f32 * 34.0);
    let bg_x = x - 10.0;
    let bg_y = y - 22.0;
    let bg_w = if x < 100.0 { 690.0 } else { 470.0 };
    let row_h = 30.0;
    draw_rectangle(
        bg_x,
        bg_y,
        bg_w,
        row_h,
        if selected {
            Color::from_rgba(83, 58, 29, 225)
        } else {
            Color::from_rgba(22, 20, 18, 175)
        },
    );
    if selected {
        draw_rectangle_lines(bg_x, bg_y, bg_w, row_h, 2.0, candle_color());
    }
    draw_ui_text(
        &format!("[{}] {}", i + 1, adv.name),
        x,
        y,
        18.0,
        text_color(),
    );
    draw_ui_text(
        &format!("{:?}", adv.class),
        x + 220.0,
        y,
        15.0,
        muted_text_color(),
    );
    draw_ui_text(
        readiness_label(adv),
        x + 330.0,
        y,
        16.0,
        readiness_color(adv),
    );
}

fn draw_facility_card(i: usize, building: &Building, selected: bool, can_build: bool) {
    let (x, y, w, h) = facility_card_rect(i);
    draw_rectangle(
        x,
        y,
        w,
        h,
        if selected {
            Color::from_rgba(77, 52, 26, 230)
        } else {
            Color::from_rgba(23, 21, 19, 218)
        },
    );
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        if selected { 2.0 } else { 1.0 },
        if selected {
            candle_color()
        } else {
            border_color()
        },
    );
    draw_ui_text(
        &building.name.to_uppercase(),
        x + 14.0,
        y + 28.0,
        18.0,
        title_color(),
    );
    draw_ui_text(
        facility_purpose(&building.id),
        x + 14.0,
        y + 52.0,
        13.0,
        muted_text_color(),
    );
    let status = if building.built {
        "Status: Built".to_string()
    } else {
        format!(
            "Cost: {}g / {}s",
            building.cost_gold, building.cost_supplies
        )
    };
    draw_ui_text(
        &status,
        x + 14.0,
        y + h - 20.0,
        15.0,
        if building.built {
            ready_color()
        } else if can_build {
            candle_color()
        } else {
            danger_color()
        },
    );
    draw_ui_text(
        if building.built { "Active" } else { "[Build]" },
        x + w - 82.0,
        y + h - 20.0,
        15.0,
        if building.built {
            muted_text_color()
        } else {
            candle_color()
        },
    );
}

fn draw_action_button(label: &str, x: f32, y: f32, w: f32, h: f32, enabled: bool) {
    let hovered = crate::ui::is_mouse_over(x, y, w, h);
    let fill = if !enabled {
        Color::from_rgba(31, 28, 25, 200)
    } else if hovered {
        Color::from_rgba(114, 78, 32, 245)
    } else {
        Color::from_rgba(74, 52, 28, 235)
    };
    draw_rectangle(x, y, w, h, fill);
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        1.0,
        if enabled {
            candle_color()
        } else {
            border_color()
        },
    );
    let tw = measure_ui_text(label, None, 16, 1.0).width;
    draw_ui_text(
        label,
        x + (w - tw) / 2.0,
        y + 21.0,
        16.0,
        if enabled {
            text_color()
        } else {
            muted_text_color()
        },
    );
}

fn draw_adventurer_details(adv: &Adventurer) {
    draw_ui_text(
        &adv.name.to_uppercase(),
        48.0,
        DETAIL_Y + 48.0,
        28.0,
        title_color(),
    );
    draw_ui_text(
        &format!("{:?} - Level {}", adv.class, adv.level),
        48.0,
        DETAIL_Y + 78.0,
        18.0,
        muted_text_color(),
    );
    let injuries = if adv.injuries.is_empty() {
        "None".to_string()
    } else {
        adv.injuries
            .iter()
            .map(|injury| injury.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let trauma = if adv.traumas.is_empty() {
        "None".to_string()
    } else {
        adv.traumas
            .iter()
            .map(|trauma| trauma.name())
            .collect::<Vec<_>>()
            .join(", ")
    };
    draw_ui_text(
        &format!(
            "HP {}/{}    Stress {}    Injuries: {}    Trauma: {}    Deck: {} cards",
            adv.hp,
            adv.max_hp,
            adv.stress,
            injuries,
            trauma,
            deck_size(adv)
        ),
        48.0,
        DETAIL_Y + 112.0,
        18.0,
        text_color(),
    );
    draw_ui_text("Best Use", 48.0, DETAIL_Y + 150.0, 18.0, candle_color());
    draw_wrapped_text(
        class_guidance(adv),
        145.0,
        DETAIL_Y + 150.0,
        screen_width() - 190.0,
        16.0,
        muted_text_color(),
    );
    draw_ui_text(
        "Actions: [View Deck] [Assign to Party] [Treat] [Train]",
        48.0,
        DETAIL_Y + 196.0,
        17.0,
        ready_color(),
    );
}

fn draw_building_details(building: &Building, kingdom: &KingdomState, can_build: bool) {
    draw_ui_text(
        &building.name.to_uppercase(),
        48.0,
        DETAIL_Y + 48.0,
        28.0,
        title_color(),
    );
    draw_ui_text(
        if building.built {
            "Constructed"
        } else {
            "Not Constructed"
        },
        48.0,
        DETAIL_Y + 78.0,
        18.0,
        if building.built {
            ready_color()
        } else {
            danger_color()
        },
    );
    draw_ui_text("Purpose", 48.0, DETAIL_Y + 116.0, 18.0, candle_color());
    draw_wrapped_text(
        facility_purpose(&building.id),
        145.0,
        DETAIL_Y + 116.0,
        screen_width() - 190.0,
        16.0,
        text_color(),
    );
    let cost_or_use = if building.built {
        facility_unlocks(&building.id).to_string()
    } else {
        format!(
            "Build Cost: {} Gold, {} Supplies. {}",
            building.cost_gold,
            building.cost_supplies,
            if can_build {
                "Enough resources."
            } else if kingdom.stats.gold < building.cost_gold {
                "Need more gold."
            } else {
                "Need more supplies."
            }
        )
    };
    draw_wrapped_text(
        &cost_or_use,
        48.0,
        DETAIL_Y + 158.0,
        screen_width() - 96.0,
        16.0,
        muted_text_color(),
    );
    draw_ui_text(
        if building.built {
            "Actions: facility active"
        } else {
            "Action: [Build Facility]"
        },
        48.0,
        DETAIL_Y + 206.0,
        17.0,
        if can_build || building.built {
            ready_color()
        } else {
            danger_color()
        },
    );
}

fn draw_goals_detail(kingdom: &KingdomState) {
    draw_ui_text(
        "CURRENT FRONTIER CHARTER",
        48.0,
        DETAIL_Y + 48.0,
        26.0,
        title_color(),
    );
    let mut y = DETAIL_Y + 84.0;
    for (quest, done) in kingdom.quest_log() {
        draw_ui_text(
            &format!("{} {}", if done { "[x]" } else { "[ ]" }, quest),
            48.0,
            y,
            18.0,
            if done { ready_color() } else { text_color() },
        );
        y += 28.0;
    }
}

fn draw_graveyard_detail(roster: &Roster) {
    draw_ui_text(
        "LOSSES AND SCARS",
        48.0,
        DETAIL_Y + 48.0,
        26.0,
        title_color(),
    );
    let text = if roster.graveyard.is_empty() {
        "The graveyard is empty, but the ledger has space."
    } else {
        "The dead stay here. Their absence should shape the next expedition."
    };
    draw_wrapped_text(
        text,
        48.0,
        DETAIL_Y + 86.0,
        screen_width() - 96.0,
        18.0,
        muted_text_color(),
    );
}

fn draw_training_detail(adv: &Adventurer, kingdom: &KingdomState) {
    draw_ui_text(
        "DECK / TRAINING",
        48.0,
        DETAIL_Y + 48.0,
        26.0,
        title_color(),
    );
    draw_ui_text(
        &format!("{} currently has {} cards.", adv.name, deck_size(adv)),
        48.0,
        DETAIL_Y + 84.0,
        18.0,
        text_color(),
    );
    draw_wrapped_text(
        if kingdom.has_building("foundry") {
            "Spend Knowledge to add the next class card. Training should feel like trading scarce insight for survival."
        } else {
            "The Foundry is required before stronger cards can be learned."
        },
        48.0,
        DETAIL_Y + 122.0,
        screen_width() - 96.0,
        17.0,
        muted_text_color(),
    );
}

fn draw_card_frame(card: &crate::combat::Card, x: f32, y: f32, w: f32, h: f32, selected: bool) {
    let accent = card_accent(card);
    draw_rectangle(x, y, w, h, Color::from_rgba(19, 17, 15, 245));
    draw_rectangle_lines(x, y, w, h, if selected { 3.0 } else { 2.0 }, accent);
    draw_rectangle(
        x + 6.0,
        y + 6.0,
        w - 12.0,
        24.0,
        Color::from_rgba(38, 33, 27, 240),
    );
    draw_ui_text(
        &format!("{}", card.cost),
        x + 12.0,
        y + 24.0,
        18.0,
        candle_color(),
    );
    draw_ui_text(
        card_type(card),
        x + w - 62.0,
        y + 24.0,
        14.0,
        muted_text_color(),
    );
    draw_rectangle(
        x + 8.0,
        y + 36.0,
        w - 16.0,
        h * 0.42,
        Color::from_rgba(43, 40, 38, 255),
    );
    draw_ui_text(&card.name, x + 10.0, y + h - 52.0, 15.0, text_color());
    draw_wrapped_text(
        &card.description,
        x + 10.0,
        y + h - 32.0,
        w - 20.0,
        12.0,
        muted_text_color(),
    );
}

fn draw_shortcuts() {
    draw_ui_text(
        "Shortcuts: 1-9 Select - Tab Tabs - M Party - D Deck - H/T Treat - U Train - F5 Save - F9 Load",
        SIDE_PAD,
        screen_height() - 18.0,
        14.0,
        muted_text_color(),
    );
}

fn panel(x: f32, y: f32, w: f32, h: f32, title: &str) {
    draw_rectangle(x, y, w, h, Color::from_rgba(16, 13, 11, 218));
    draw_rectangle_lines(x, y, w, h, 1.0, border_color());
    draw_rectangle(x, y, w, 32.0, Color::from_rgba(43, 30, 17, 205));
    draw_ui_text(title, x + 14.0, y + 23.0, 16.0, candle_color());
}

fn draw_wrapped_text(text: &str, x: f32, y: f32, max_width: f32, font_size: f32, color: Color) {
    let mut current = String::new();
    let mut line_y = y;
    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if measure_ui_text(&candidate, None, font_size as u16, 1.0).width > max_width
            && !current.is_empty()
        {
            draw_ui_text(&current, x, line_y, font_size, color);
            current = word.to_string();
            line_y += font_size + 5.0;
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        draw_ui_text(&current, x, line_y, font_size, color);
    }
}

fn number_key(i: usize) -> Option<KeyCode> {
    match i {
        0 => Some(KeyCode::Key1),
        1 => Some(KeyCode::Key2),
        2 => Some(KeyCode::Key3),
        3 => Some(KeyCode::Key4),
        4 => Some(KeyCode::Key5),
        5 => Some(KeyCode::Key6),
        6 => Some(KeyCode::Key7),
        7 => Some(KeyCode::Key8),
        8 => Some(KeyCode::Key9),
        _ => None,
    }
}

fn tab_width(tab: BaseTab) -> f32 {
    match tab {
        BaseTab::DeckTraining => 126.0,
        BaseTab::Graveyard => 92.0,
        BaseTab::Buildings => 88.0,
        _ => 78.0,
    }
}

fn action_buttons() -> [&'static str; 6] {
    [
        "Embark",
        "Roster",
        "Facilities",
        "Treat",
        "Recruit",
        "Decks",
    ]
}

fn action_enabled(
    action: &str,
    kingdom: &KingdomState,
    roster: &Roster,
    selected_adventurer: Option<usize>,
) -> bool {
    match action {
        "Embark" => !roster.adventurers.is_empty(),
        "Treat" | "Decks" => selected_adventurer
            .and_then(|idx| roster.adventurers.get(idx))
            .is_some(),
        "Recruit" => kingdom.has_building("guild_hall"),
        _ => true,
    }
}

fn adventurer_row_rect(i: usize) -> (f32, f32, f32, f32) {
    (285.0, MAIN_Y + 50.0 + (i as f32 * 34.0) - 22.0, 470.0, 30.0)
}

fn roster_adventurer_row_rect(i: usize) -> (f32, f32, f32, f32) {
    (34.0, MAIN_Y + 52.0 + (i as f32 * 34.0) - 22.0, 690.0, 30.0)
}

fn adventurer_row_hit_rect(active_tab: BaseTab, i: usize) -> (f32, f32, f32, f32) {
    if active_tab == BaseTab::Roster {
        roster_adventurer_row_rect(i)
    } else {
        adventurer_row_rect(i)
    }
}

fn detail_back_button_rect() -> (f32, f32, f32, f32) {
    (screen_width() - 168.0, DETAIL_Y + 4.0, 126.0, 30.0)
}

fn party_mission_button_rect() -> (f32, f32, f32, f32) {
    (48.0, MAIN_Y + 420.0, 190.0, 34.0)
}

fn party_back_button_rect() -> (f32, f32, f32, f32) {
    (254.0, MAIN_Y + 420.0, 150.0, 34.0)
}

fn facility_card_rect(i: usize) -> (f32, f32, f32, f32) {
    let cols = 3;
    let card_w = (screen_width() - 84.0) / cols as f32;
    let card_h = 92.0;
    let col = i % cols;
    let row = i / cols;
    (
        SIDE_PAD + 14.0 + (col as f32 * (card_w + 12.0)),
        MAIN_Y + 48.0 + (row as f32 * (card_h + 12.0)),
        card_w,
        card_h,
    )
}

fn deck_size(adv: &Adventurer) -> usize {
    let class_name = format!("{:?}", adv.class);
    crate::combat::Card::load_deck_for_class(&class_name, &adv.deck_additions).len()
}

fn readiness_label(adv: &Adventurer) -> &'static str {
    if adv.hp <= adv.max_hp / 3 {
        "Needs Rest"
    } else if adv.stress >= 75 {
        "Fracturing"
    } else if adv.stress >= 50 {
        "Stressed"
    } else if !adv.injuries.is_empty() {
        "Wounded"
    } else {
        "Ready"
    }
}

fn readiness_color(adv: &Adventurer) -> Color {
    match readiness_label(adv) {
        "Ready" => ready_color(),
        "Stressed" | "Wounded" => candle_color(),
        _ => danger_color(),
    }
}

fn morale_label(value: i32) -> &'static str {
    match value {
        v if v >= 75 => "Steady",
        v if v >= 45 => "Fragile",
        v if v >= 20 => "Shaky",
        _ => "Breaking",
    }
}

fn morale_color(value: i32) -> Color {
    if value >= 60 {
        ready_color()
    } else if value >= 30 {
        candle_color()
    } else {
        danger_color()
    }
}

fn facility_purpose(id: &str) -> &'static str {
    match id {
        "infirmary" => "Heal injuries before they become permanent.",
        "chapel" => "Reduce stress and prevent resolve collapse.",
        "foundry" => "Improve equipment and unlock stronger cards.",
        "guild_hall" => "Recruit, dismiss, and train adventurers.",
        "watchtowers" => "Lower threat and unlock scouting missions.",
        "citadel" => "Final objective and win condition.",
        _ => "Frontier support facility.",
    }
}

fn facility_unlocks(id: &str) -> &'static str {
    match id {
        "infirmary" => "Unlocks: Treat Wounds and safer injury recovery.",
        "chapel" => "Unlocks: Stress relief before resolve collapse.",
        "foundry" => "Unlocks: Knowledge-based card training.",
        "guild_hall" => "Unlocks: Recruitment and roster growth.",
        "watchtowers" => "Unlocks: Ruined Outpost scouting routes.",
        "citadel" => "Secures the campaign ending.",
        _ => "Facility active.",
    }
}

fn class_guidance(adv: &Adventurer) -> &'static str {
    match format!("{:?}", adv.class).as_str() {
        "Soldier" => "Strong frontline fighter. Good for Suppress and Combat-heavy missions.",
        "Scout" => "Route finder and opportunist. Good when the mission may punish slow choices.",
        "Healer" => "Keeps the party alive and calmer. Best when wounds or stress are expected.",
        "Mystic" => {
            "High-impact control and burst damage. Best when dangerous enemies must be disrupted."
        }
        _ => "Reliable frontier hand. Match them to current wounds, stress, and route risk.",
    }
}

fn card_type(card: &crate::combat::Card) -> &'static str {
    if card.is_attack() {
        "Attack"
    } else if card
        .effects
        .iter()
        .any(|effect| matches!(effect, crate::combat::CardEffect::Heal(_)))
    {
        "Heal"
    } else if card
        .effects
        .iter()
        .any(|effect| matches!(effect, crate::combat::CardEffect::Block(_)))
    {
        "Guard"
    } else {
        "Skill"
    }
}

fn card_accent(card: &crate::combat::Card) -> Color {
    match card_type(card) {
        "Attack" => danger_color(),
        "Guard" => info_color(),
        "Heal" => ready_color(),
        _ => candle_color(),
    }
}

fn table_color() -> Color {
    Color::from_rgba(40, 25, 16, 255)
}

fn title_color() -> Color {
    Color::from_rgba(236, 224, 198, 255)
}

fn text_color() -> Color {
    Color::from_rgba(220, 212, 190, 255)
}

fn muted_text_color() -> Color {
    Color::from_rgba(164, 153, 130, 255)
}

fn parchment_color() -> Color {
    Color::from_rgba(196, 176, 130, 255)
}

fn candle_color() -> Color {
    Color::from_rgba(214, 154, 62, 255)
}

fn ready_color() -> Color {
    Color::from_rgba(112, 143, 92, 255)
}

fn danger_color() -> Color {
    Color::from_rgba(150, 55, 48, 255)
}

fn info_color() -> Color {
    Color::from_rgba(114, 136, 146, 255)
}

fn mystery_color() -> Color {
    Color::from_rgba(119, 86, 132, 255)
}

fn border_color() -> Color {
    Color::from_rgba(108, 82, 51, 210)
}
