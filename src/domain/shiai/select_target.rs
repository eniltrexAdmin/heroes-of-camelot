use std::collections::HashMap;
use super::*;

// TODO something like "target strategy"
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
        AttackParty(_) => get_attacker_alive_team(current_state),
        DefenseParty(_) => get_defender_alive_team(current_state),
    }
}

fn get_attacker_alive_team(current_state: &HashMap<ShiaiPosition, BattleTeam>) -> ShiaiPosition {
    if let Some(captain_team) = current_state.get(&DefenseParty(CaptainTeam)) {
        if captain_team.is_alive() {
            DefenseParty(CaptainTeam);
        }
    } else if let Some(second_team) = current_state.get(&DefenseParty(SecondTeam)) {
        if second_team.is_alive() {
            DefenseParty(SecondTeam);
        }
    } else if let Some(third_team) = current_state.get(&DefenseParty(ThirdTeam)) {
        if third_team.is_alive() {
            DefenseParty(ThirdTeam);
        }
    }
    panic!("all teams are dead.")
}

fn get_defender_alive_team(current_state: &HashMap<ShiaiPosition, BattleTeam>) -> ShiaiPosition {
    if let Some(captain_team) = current_state.get(&AttackParty(CaptainTeam)) {
        if captain_team.is_alive() {
            AttackParty(CaptainTeam);
        }
    } else if let Some(second_team) = current_state.get(&AttackParty(SecondTeam)) {
        if second_team.is_alive() {
            AttackParty(SecondTeam);
        }
    } else if let Some(third_team) = current_state.get(&AttackParty(ThirdTeam)) {
        if third_team.is_alive() {
            AttackParty(ThirdTeam);
        }
    }
    panic!("all teams are dead.")
}
