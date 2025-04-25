use std::collections::HashMap;
use crate::domain::*;
use super::*;

pub enum ShiaiEvent{
    TeamAttacked(TeamAttacked),
    DamageReceived(DamageReceived)
}

pub struct ShiaiResult {
    winner: Party,
    shiai_events: Vec<ShiaiEvent>,
    teams: HashMap<ShiaiPosition, BattleTeam>,
}

pub fn battle(attacker: Party, defender: Party) -> ShiaiResult {
    let mut events = Vec::new();
    let mut turn = 1;


    let mut shiai = start_state(attacker.clone(), defender.clone());


    while turn < 30 {
        let active_team = select_active_team(&attacker, &defender, turn);

        play_turn(&mut shiai, active_team);

        turn = turn + 1;
    }
    ShiaiResult{
        shiai_events: events,
        winner: attacker,
        teams: shiai.clone(),
    }
}

fn start_state(attacker: Party, defender: Party) -> HashMap<ShiaiPosition, BattleTeam> {
    let mut teams = HashMap::new();
    teams.insert(AttackParty(CaptainTeam), BattleTeam::new(
        attacker.captain_team().clone(), AttackParty(CaptainTeam)
    ));
    teams
}

fn play_turn(state: &mut HashMap<ShiaiPosition, BattleTeam>, subject: ShiaiPosition) {
    // combo skills part
    // active skills part
    attack(state, subject);
}



fn select_active_team(attacker: &Party, defender: &Party, turn: u8) -> ShiaiPosition {
    match turn %2 {
        1 => AttackParty(CaptainTeam),
        0 => DefenseParty(CaptainTeam),
        _ => unreachable!()
    }
}

// fn select_team(party: &Party, turn: u8) -> TeamPosition {
//     let mut available_teams = vec![CaptainTeam];
//     if party.second_team().is_some() {
//         available_teams.push(SecondTeam);
//     }
//     if party.third_team().is_some() {
//         available_teams.push(ThirdTeam);
//     }
//
//     let index = (turn as usize) % available_teams.len();
//     available_teams[index].clone()
// }

#[cfg(test)]
mod tests {
    use crate::data::stub_party;
    use super::*;

    #[test]
    fn test_select_active_team() {
        let attacker = stub_party();
        let defender = stub_party();
        let selected_team = select_active_team(&attacker, &defender, 35);

        assert_eq!(DefenseParty(CaptainTeam), selected_team);

    }
    // #[test]
    // fn play_battle() {
    //     let attacker = stub_party();
    //     let defender = stub_party();
    //     let engine = ShiaiEngine::new(attacker, defender);
    //
    //     let result = engine.battle();
    //
    // }
}