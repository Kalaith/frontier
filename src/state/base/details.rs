//! Detail panel and the per-entity detail readouts it renders.

use super::helpers::{
    candle_color, class_guidance, danger_color, deck_size, detail_back_button_rect,
    draw_wrapped_text, facility_purpose, facility_unlocks, muted_text_color, panel, ready_color,
    text_color, title_color,
};
use super::panels::draw_action_button;
use super::{BaseState, BaseTab, DETAIL_Y, SIDE_PAD};
use crate::kingdom::{Adventurer, Building, KingdomState, Roster};
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

impl BaseState {
    pub(super) fn draw_detail_panel(&self, kingdom: &KingdomState, roster: &Roster) {
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
}

pub(super) fn draw_adventurer_details(adv: &Adventurer) {
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

pub(super) fn draw_building_details(building: &Building, kingdom: &KingdomState, can_build: bool) {
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

pub(super) fn draw_goals_detail(kingdom: &KingdomState) {
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

pub(super) fn draw_graveyard_detail(roster: &Roster) {
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

pub(super) fn draw_training_detail(adv: &Adventurer, kingdom: &KingdomState) {
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
