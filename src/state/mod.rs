//! Game state modules
//! 
//! Each state represents a distinct game mode with its own update/draw logic.

mod base;
mod mission;
mod mission_select;
mod combat;
mod results;

pub use base::BaseState;
pub use mission::MissionState;
pub use mission_select::MissionSelectState;
pub use combat::{CombatState, MissionContext};
pub use results::ResultState;

/// Explicit state transitions - no magic callbacks
pub enum StateTransition {
    ToBase,
    ToMissionSelect(MissionSelectState),
    ToMission(MissionState),
    ToCombat(CombatState),
    ToResults(ResultState),
}
