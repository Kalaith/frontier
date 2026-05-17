//! Combat system modules
//!
//! Cards emit effects; systems resolve them. Cards never directly mutate state.

mod card;
mod effects;
mod resolver;
mod unit;

pub use card::{Card, CardClass};
pub use effects::CardEffect;
pub use resolver::CombatResolver;
pub use unit::{EnemyIntent, Unit};
