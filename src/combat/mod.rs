//! Combat system modules
//! 
//! Cards emit effects; systems resolve them. Cards never directly mutate state.

mod unit;
mod card;
mod effects;
mod resolver;

pub use unit::{Unit, EnemyIntent};
pub use card::{Card, STARTER_DECK_IDS};
pub use effects::CardEffect;
pub use resolver::CombatResolver;
