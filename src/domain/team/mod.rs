pub mod team;
pub use team::TeamType::*;
pub use team::*;
mod team_attack;
use team_attack::TeamAttack;
pub mod team_hp;
use team_hp::TeamHealthPoints;
pub mod team_factory;
pub use team_factory::*;

