use crate::domain::*;
use crate::domain::battle::battle_team::attack::*;

#[derive(Clone, Debug)]
pub struct TeamAttackedDomainEvent{
    pub attacker: BattlePosition,
    pub target: BattlePosition,
    pub damage_received: Damage
}


impl BattleTeam{
    pub fn apply_team_attacked_domain_event(
        &mut self,
        team_attacked_domain_event: TeamAttackedDomainEvent)
    {
        self.current_hp = match team_attacked_domain_event.damage_received {
            Damage::Physical(physical_damage) => self.current_hp().apply_damage(physical_damage),
            Damage::Magical => self.current_hp().clone(),
        }
    }
}