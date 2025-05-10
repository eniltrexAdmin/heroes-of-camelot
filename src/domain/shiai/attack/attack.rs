use crate::domain::{Shiai, ShiaiAction, ShiaiCommand, ShiaiError, ShiaiEvent, ShiaiPosition};
use crate::domain::shiai::select_target::{select_target, TargetStrategy};
use crate::domain::shiai::shiai_state::ShiaiState;

#[derive(Clone, Debug)]
pub struct AttackCommand {
    pub subject: ShiaiPosition,
    pub target: ShiaiPosition,
}

impl AttackCommand {
    fn new(shiai: &ShiaiState, subject: ShiaiPosition) -> Self {
        let target = select_target(&shiai.state, subject.clone(), TargetStrategy::Default);
        Self { subject, target }
    }
}

fn attack_result(shiai: &ShiaiState, command: AttackCommand) -> Result<Vec<ShiaiEvent>, ShiaiError> {
    let attacker = shiai.state.get(&command.subject)
        .ok_or(ShiaiError::SubjectMissingError)?;
    let target = shiai.state.get(&command.target)
        .ok_or(ShiaiError::TargetMissingError)?;

    Ok(target.receive_attack(attacker))
}


pub fn attack_action(shiai: &ShiaiState, subject: ShiaiPosition) -> Result<ShiaiAction, ShiaiError>{
    let attack_command = AttackCommand::new(&shiai, subject);
    let plan_result = attack_result(&shiai, attack_command.clone())?;

    Ok(ShiaiAction::new(ShiaiCommand::Attack(attack_command), plan_result))
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

        let shiai = Shiai::new(attacker, defender);

        let result = attack_action(&shiai.current_state, AttackParty(CaptainTeam));
        assert!(result.is_ok());
        let result = result.unwrap();
        if let ShiaiCommand::Attack(attack_command) = &result.command {
            assert_eq!(attack_command.target, DefenseParty(CaptainTeam));
        }
    }
}