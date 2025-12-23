//! Combat system modules
//! 
//! Cards emit effects; systems resolve them. Cards never directly mutate state.

mod unit;
mod card;
mod effects;
mod resolver;

pub use unit::Unit;
pub use card::Card;
pub use effects::CardEffect;
pub use resolver::CombatResolver;
