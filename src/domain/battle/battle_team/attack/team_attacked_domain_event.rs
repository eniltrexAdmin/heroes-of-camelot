use super::*;

#[derive(Clone, Debug)]
pub struct TeamAttackedDomainEvent{
    pub attacker: BattlePosition,
    pub target: BattlePosition,
    pub damage_received: AttackDamage
}