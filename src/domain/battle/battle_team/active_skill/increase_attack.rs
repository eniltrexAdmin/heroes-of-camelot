use crate::domain::{BattlePosition, BattleState, SkillBaseValue, Target};
use crate::domain::battle::battle_team::BattleTeam;

#[derive(Clone, Debug)]
pub struct IncreasedThisTurnAttackDomainEvent {
    pub target: BattlePosition,
    pub increase_by: AttackIncreaseValue,
}

impl Target for IncreasedThisTurnAttackDomainEvent{
    fn target(&self) -> BattlePosition {
        self.target.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AttackIncreaseValue(u128);


impl AttackIncreaseValue {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

pub fn increase_turn_attack(_state: &BattleState, subject: BattlePosition, base_value: SkillBaseValue)
                            -> IncreasedThisTurnAttackDomainEvent {
    let value_increased = AttackIncreaseValue::new(base_value.value());
    // more modifiers depending on battle state, increase more/less, etc.
    IncreasedThisTurnAttackDomainEvent {
        target: subject,
        increase_by: value_increased,
    }
}

impl BattleTeam {
    pub fn apply_attack_increased(
        &mut self,
        team_attacked_domain_event: IncreasedThisTurnAttackDomainEvent) {
        self.current_attack = self.current_attack().increase(team_attacked_domain_event.increase_by)
    }
}

// TODO test module