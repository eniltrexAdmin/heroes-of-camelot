use std::collections::HashMap;
use super::*;

pub enum TargetStrategy {
    Own,
    Default,
}

pub fn select_target(current_state: &HashMap<ShiaiPosition, BattleTeam>, subject_team: ShiaiPosition, strategy: TargetStrategy) -> ShiaiPosition {
    match strategy {
        _ => select_default_target(current_state, subject_team),
    }
}

fn select_default_target(current_state: &HashMap<ShiaiPosition, BattleTeam>, current_team: ShiaiPosition) -> ShiaiPosition {
    match current_team {
        AttackParty(_) => get_defense_party_alive_team(current_state),
        DefenseParty(_) => get_attack_party_alive_team(current_state),
    }
}

fn get_defense_party_alive_team(current_state: &HashMap<ShiaiPosition, BattleTeam>) -> ShiaiPosition {
    [DefenseParty(CaptainTeam), DefenseParty(SecondTeam), DefenseParty(ThirdTeam)]
        .into_iter()
        .find(|position| current_state.get(position).map_or(
            false, |team| team.is_alive()))
        .expect("No alive team found in defense party")

}

fn get_attack_party_alive_team(current_state: &HashMap<ShiaiPosition, BattleTeam>) -> ShiaiPosition {
    [AttackParty(CaptainTeam), AttackParty(SecondTeam), AttackParty(ThirdTeam)]
        .into_iter()
        .find(|position| current_state.get(position).map_or(
            false, |team| team.is_alive()))
        .expect("No alive team found in defense party")
}


#[cfg(test)]
mod tests {

}