use super::*;

pub struct TeamAttackedDomainEvent{
    subject: ShiaiPosition,
    target: ShiaiPosition,
}


pub fn attack(subject: &mut BattleTeam, target: &mut BattleTeam) -> TeamAttackedDomainEvent{

    TeamAttackedDomainEvent{
        subject: subject.position().clone(),
        target: target.position().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::stub_team;
    use super::*;

    #[test]
    fn test_attack() {
        let mut battle_team = BattleTeam::new(stub_team(), AttackParty(CaptainTeam));
        let mut battle_team_2 = BattleTeam::new(stub_team(), DefenseParty(CaptainTeam));

        let result = attack(&mut battle_team, &mut battle_team_2);

        assert_eq!(
            battle_team_2.original_team().health_points().value() -
            battle_team.current_attack().value(),
            battle_team_2.current_hp().value(),
        );

        assert_eq!(battle_team.position(), &result.subject);
        assert_eq!(battle_team_2.position(), &result.target);
    }
}