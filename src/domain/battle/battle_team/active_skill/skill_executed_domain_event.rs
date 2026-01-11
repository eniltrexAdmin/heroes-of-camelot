use crate::domain::{BattlePosition, Card};
use super::*;

#[derive(Clone, Debug)]
pub struct ActiveSkillExecutedDomainEvent{
    pub card: Card, // TODO pass something more sensible than the whole card.
    pub battle_team: BattlePosition,
    pub effect: ActiveSkillExecutedEffect
}

impl Target for ActiveSkillExecutedDomainEvent {
    fn target(&self) -> BattlePosition {
        self.effect.target()
    }
}

#[derive(Clone, Debug)]
pub enum ActiveSkillExecutedEffect{
    IncreasedThisTurnAttack(IncreasedThisTurnAttackDomainEvent)
}

impl Target for ActiveSkillExecutedEffect{
    fn target(&self) -> BattlePosition {
        match self {
            ActiveSkillExecutedEffect::IncreasedThisTurnAttack(domain_event) => {
                domain_event.target()
            }
        }
    }
}

