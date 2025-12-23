//! Global game loop and state switching
//! 
//! Only one GameState is active at a time. Transitions are explicit.

use crate::state::*;
use crate::kingdom::{KingdomState, Roster};

/// Top-level game state enum - explicit state machine
pub enum GameState {
    /// Kingdom base management
    Base(BaseState),
    /// Selecting a mission to embark on
    MissionSelect(MissionSelectState),
    /// Active mission/expedition
    Mission(MissionState),
    /// Turn-based card combat
    Combat(CombatState),
    /// Post-mission results and consequences
    Results(ResultState),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Base(BaseState::default())
    }
}

/// Main game struct holding all state
pub struct Game {
    pub state: GameState,
    pub kingdom: KingdomState,
    pub roster: Roster,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            state: GameState::default(),
            kingdom: KingdomState::default(),
            roster: Roster::starter(),
        }
    }
    
    /// Update game logic based on current state
    pub fn update(&mut self) {
        match &mut self.state {
            GameState::Base(state) => {
                if let Some(transition) = state.update(&mut self.kingdom, &self.roster) {
                    self.transition(transition);
                }
            }
            GameState::MissionSelect(state) => {
                if let Some(transition) = state.update(&self.roster) {
                    self.transition(transition);
                }
            }
            GameState::Mission(state) => {
                if let Some(transition) = state.update() {
                    self.transition(transition);
                }
            }
            GameState::Combat(state) => {
                if let Some(transition) = state.update() {
                    self.transition(transition);
                }
            }
            GameState::Results(state) => {
                if let Some(transition) = state.update(&mut self.kingdom, &mut self.roster) {
                    self.transition(transition);
                }
            }
        }
    }
    
    /// Draw current state
    pub fn draw(&self) {
        match &self.state {
            GameState::Base(state) => state.draw(&self.kingdom, &self.roster),
            GameState::MissionSelect(state) => state.draw(),
            GameState::Mission(state) => state.draw(),
            GameState::Combat(state) => state.draw(),
            GameState::Results(state) => state.draw(),
        }
    }
    
    /// Handle explicit state transitions
    fn transition(&mut self, transition: StateTransition) {
        self.state = match transition {
            StateTransition::ToBase => GameState::Base(BaseState::default()),
            StateTransition::ToMissionSelect(select) => GameState::MissionSelect(select),
            StateTransition::ToMission(mission) => GameState::Mission(mission),
            StateTransition::ToCombat(combat) => GameState::Combat(combat),
            StateTransition::ToResults(results) => GameState::Results(results),
        };
    }
}
