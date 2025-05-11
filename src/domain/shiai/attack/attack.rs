use crate::domain::*;
use crate::domain::TargetStrategy::Default;
use super::*;


impl ShiaiAction {
    pub fn new_attack_action(subject: ShiaiPosition, events: Vec<ShiaiEvent>) -> Self {
        Self{
            subject,
            command: ShiaiCommandType::Attack,
            events,
        }
    }
}

pub fn attack_action(shiai: ShiaiState, subject: ShiaiPosition)
    -> Result<(ShiaiState, ShiaiAction), ShiaiError>{
    let target = select_target(&shiai, &subject, Default);
    let plan_result = attack_result(&shiai, subject.clone(), target)?;

    let updated_shiai = shiai.apply_domain_events(plan_result.clone())?;

    Ok((updated_shiai, ShiaiAction::new_attack_action(subject, plan_result)))
}




fn attack_result(shiai: &ShiaiState, attacker: ShiaiPosition, target: ShiaiPosition)
    -> Result<Vec<ShiaiEvent>, ShiaiError> {
    let attacker = shiai.state.get(&attacker)
        .ok_or(ShiaiError::SubjectMissingError)?;
    let target = shiai.state.get(&target)
        .ok_or(ShiaiError::TargetMissingError)?;

    Ok(target.receive_attack(attacker))
}






#[cfg(test)]
mod tests {
    use crate::data::{stub_party, stub_party_2};
    use crate::domain::{AttackParty,  CaptainTeam, DefenseParty};
    use super::*;

    #[test]
    fn test_attack_action() {
        let attacker = stub_party();
        let defender = stub_party_2();

        let shiai = Shiai::new(attacker.clone(), defender);

        let result = attack_action(&shiai.current_state, AttackParty(CaptainTeam));
        assert!(result.is_ok());
        let result = result.unwrap();
        let resulting_event = result.events.first().unwrap();
        assert_eq!(DefenseParty(CaptainTeam), resulting_event.target);
        if let ShiaiEventType::DamageReceived(damage) = &resulting_event.event {
            assert_eq!(attacker.captain_team().attack().value(), damage.value());
        }
    }
}