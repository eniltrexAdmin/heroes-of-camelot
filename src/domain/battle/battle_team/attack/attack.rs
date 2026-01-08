use crate::domain::*;
use super::*;



impl BattleTeam{
    pub fn attack(&self, state: &BattleState)
                         -> Vec<BattleEvent> {
        let target_pos = select_target(&state, self.position(), TargetStrategy::Default);
        let target = state.expect_team(&target_pos).clone();

        let event = TeamAttacked(TeamAttackedDomainEvent {
            attacker: self.position().clone(),
            target: target_pos,
            damage_received: target.calculate_attack_damage(self.current_attack().clone()),
        });
        vec![event]
    }
}




impl BattleState {
    pub fn apply_team_attacked_domain_event(&mut self, event: TeamAttackedDomainEvent)
    {
        if let Some(target_team) = self.state.get_mut(&event.target) {
            target_team.apply_team_attacked_domain_event(event);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::data::{shiai_state_stub, stub_party, stub_party_2};
    use crate::domain::{AttackParty,  CaptainTeam, DefenseParty};
    use super::*;

    #[test]
    fn test_attack_action() {
        let mut shiai_state = shiai_state_stub();
        let target_life = shiai_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value();
        let attacker_attack = shiai_state.expect_team(&AttackParty(CaptainTeam)).current_attack().value();

        let  domain_event = handle_attack(&shiai_state, AttackParty(CaptainTeam));


        assert_eq!(1, domain_event.len());

        shiai_state.apply_domain_events(domain_event);
        assert_eq!(
            target_life - attacker_attack,
            shiai_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value()
        );
    }
}