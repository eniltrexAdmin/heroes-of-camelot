use crate::domain::{BattlePosition, BattleTeam, Card};
use super::*;

#[derive(Clone, Debug)]
pub struct IncreasedThisTurnAttackDomainEvent{
    pub card: Card, // for animation, maybe just add number of order
    pub target: BattlePosition,
    pub increase_by: AttackIncreaseValue
}

