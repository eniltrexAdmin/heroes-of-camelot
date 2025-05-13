use crate::domain::*;
use crate::domain::shiai::shiai_state::ShiaiEvent;
use crate::domain::TargetStrategy::Default;
use super::*;

#[derive(Clone, Debug)]
pub struct TeamAttackedDomainEvent{
    attacker: ShiaiPosition,
    target: ShiaiPosition,
    damage_received: Damage
}

pub fn attack(state: ShiaiState, attacker_pos: ShiaiPosition)
     -> (ShiaiState, Vec<ShiaiEvent>) {
    let attacker = state.expect_team(&attacker_pos).clone();
    let target_pos = select_target(&state, &attacker_pos, Default);
    let target = state.expect_team(&target_pos).clone();

    let event = ShiaiEvent::Attack(TeamAttackedDomainEvent {
        attacker: attacker_pos,
        target: target_pos,
        damage_received: target.calculate_attack_damage(attacker.current_attack().clone()),
    });

    let new_state = state.apply_domain_events(
        vec![event.clone()]);

    (new_state, vec![event])
}

impl ShiaiState {
    pub fn apply_team_attacked_domain_event(self, event: TeamAttackedDomainEvent)
        -> ShiaiState {
        let mut new_state = self.clone();
        let target = self.expect_team(&event.target).clone()
            .receive_damage(event.damage_received);
        new_state.state.insert(target.position().clone(), target);
        new_state
    }
}


#[cfg(test)]
mod tests {
    use crate::data::{shiai_state_stub, stub_party, stub_party_2};
    use crate::domain::{AttackParty,  CaptainTeam, DefenseParty};
    use super::*;

    #[test]
    fn test_attack_action() {
        let shiai_state = shiai_state_stub();
        let target_life = shiai_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value();
        let attacker_attack = shiai_state.expect_team(&AttackParty(CaptainTeam)).current_attack().value();

        let (new_state, domain_event) = attack(shiai_state, AttackParty(CaptainTeam));


        assert_eq!(1, domain_event.len());
        assert_eq!(
            target_life - attacker_attack,
            new_state.expect_team(&DefenseParty(CaptainTeam)).current_hp().value()
        );
    }
}