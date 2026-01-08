use crate::domain::*;
use super::*;

pub fn increase_turn_attack(card: &Card, target: BattlePosition, formula: &SkillEffectValueFormula )
    -> IncreasedThisTurnAttackDomainEvent {
    // extra logic if necessary

    IncreasedThisTurnAttackDomainEvent{
        card: card.clone(),
        target,
        increase_by:AttackIncreaseValue::new(formula, card),
    }
}