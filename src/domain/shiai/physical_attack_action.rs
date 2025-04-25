use std::collections::HashMap;
use super::*;

pub struct TeamAttacked {
    subject: ShiaiPosition,
    target: ShiaiPosition,
}

pub fn attack(state:  &mut HashMap<ShiaiPosition, BattleTeam>, subject: ShiaiPosition) -> TeamAttacked {
    let target =   select_target(&state, subject.clone(), TargetStrategy::Default);

    // TODO this can panic or something if its the same target
    let [subject_team, target_team] = state.get_disjoint_mut([&subject, &target]);
    attack_team(
        subject_team.unwrap(),
        target_team.unwrap()
    )
}



fn attack_team(subject: &mut BattleTeam, target: &mut BattleTeam) -> TeamAttacked {

    TeamAttacked {
        subject: subject.position().clone(),
        target: target.position().clone()
    }
}

// pub fn self_attack(subject: &mut BattleTeam) ->TeamAttacked {}

#[cfg(test)]
mod tests {
    use crate::data::stub_team;
    use super::*;

    #[test]
    fn test_attack_team() {
        let mut battle_team = BattleTeam::new(stub_team(), AttackParty(CaptainTeam));
        let mut battle_team_2 = BattleTeam::new(stub_team(), DefenseParty(CaptainTeam));

        let result = attack_team(&mut battle_team, &mut battle_team_2);

        assert_eq!(
            battle_team_2.original_team().health_points().value() -
            battle_team.current_attack().value(),
            battle_team_2.current_hp().value(),
        );

        assert_eq!(battle_team.position(), &result.subject);
        assert_eq!(battle_team_2.position(), &result.target);
    }
}