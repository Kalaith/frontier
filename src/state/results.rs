//! Results state - post-mission consequences and resolution

use macroquad::prelude::*;
use crate::kingdom::{KingdomState, Roster};
use super::StateTransition;

/// Post-mission results state
pub struct ResultState {
    pub victory: bool,
    pub stress_gained: i32,
    pub hp_lost: i32,
    pub injuries: Vec<String>,
    pub rewards: Vec<String>,
    pub adventurer_id: String,
}

impl Default for ResultState {
    fn default() -> Self {
        Self::victory_for("")
    }
}

impl ResultState {
    pub fn victory_for(adventurer_id: &str) -> Self {
        Self {
            victory: true,
            stress_gained: 5,
            hp_lost: 0,
            injuries: vec![],
            rewards: vec!["10 Supplies".to_string(), "+5 Knowledge".to_string()],
            adventurer_id: adventurer_id.to_string(),
        }
    }
    
    pub fn defeat_for(adventurer_id: &str) -> Self {
        Self {
            victory: false,
            stress_gained: 15,
            hp_lost: 20,
            injuries: vec!["Wounded Leg".to_string()],
            rewards: vec![],
            adventurer_id: adventurer_id.to_string(),
        }
    }
    
    pub fn update(&mut self, kingdom: &mut KingdomState, roster: &mut Roster) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Enter) {
            // Apply consequences to kingdom
            if self.victory {
                kingdom.stats.supplies += 10;
                kingdom.stats.knowledge += 5;
            } else {
                kingdom.stats.morale -= 10;
            }
            
            // Apply consequences to adventurer
            if let Some(adv) = roster.get_mut(&self.adventurer_id) {
                adv.stress += self.stress_gained;
                adv.hp = (adv.hp - self.hp_lost).max(1); // Don't die from results screen
                
                // Check for stress trauma
                if adv.stress >= 100 {
                    // Could add trauma here in the future
                    adv.stress = 100;
                }
            }
            
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self) {
        let title = if self.victory { "MISSION COMPLETE" } else { "MISSION FAILED" };
        let title_color = if self.victory { GREEN } else { RED };
        
        draw_text(title, 20.0, 60.0, 36.0, title_color);
        
        let mut y = 120.0;
        
        draw_text(&format!("Stress Gained: +{}", self.stress_gained), 20.0, y, 20.0, ORANGE);
        y += 30.0;
        
        if self.hp_lost > 0 {
            draw_text(&format!("HP Lost: -{}", self.hp_lost), 20.0, y, 20.0, RED);
            y += 30.0;
        }
        
        if !self.injuries.is_empty() {
            draw_text("Injuries:", 20.0, y, 20.0, RED);
            y += 25.0;
            for injury in &self.injuries {
                draw_text(&format!("  - {}", injury), 20.0, y, 18.0, PINK);
                y += 22.0;
            }
            y += 10.0;
        }
        
        if !self.rewards.is_empty() {
            draw_text("Rewards:", 20.0, y, 20.0, GREEN);
            y += 25.0;
            for reward in &self.rewards {
                draw_text(&format!("  + {}", reward), 20.0, y, 18.0, LIME);
                y += 22.0;
            }
        }
        
        draw_text("[ENTER] Return to Kingdom", 20.0, screen_height() - 40.0, 20.0, GREEN);
    }
}
