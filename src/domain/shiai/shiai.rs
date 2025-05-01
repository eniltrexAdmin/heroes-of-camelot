use std::collections::HashMap;
use crate::domain::*;
use crate::domain::shiai::print_shiai::print_state;
use super::*;

pub enum ShiaiEvent {
    TeamAttacked(TeamAttacked),
    DamageReceived(DamageReceived),
}

pub struct ShiaiResult {
    winner: Party,
    shiai_events: Vec<ShiaiEvent>,
    teams: HashMap<ShiaiPosition, BattleTeam>,
}
impl ShiaiResult {
    pub fn teams(&self) -> &HashMap<ShiaiPosition, BattleTeam> {
        &self.teams
    }
}

pub fn battle(attacker: Party, defender: Party) -> ShiaiResult {
    let mut events = Vec::new();
    let mut turn = 1;

    let mut shiai = start_state(attacker.clone(), defender.clone());

    println!("Starting Battle:");
    print_state(&shiai);
    while turn < 5 {
        println!("Turn {}:", turn);
        let active_team = select_turn_team(turn);
        play_turn(&mut shiai, active_team);
        print_state(&shiai);
        turn = turn + 1;
    }
    ShiaiResult {
        shiai_events: events,
        winner: attacker,
        teams: shiai.clone(),
    }
}

fn start_state(attacker: Party, defender: Party) -> HashMap<ShiaiPosition, BattleTeam> {
    let mut teams = HashMap::new();
    teams.insert(AttackParty(CaptainTeam), BattleTeam::new(
        attacker.captain_team().clone(), AttackParty(CaptainTeam),
    ));
    teams.insert(DefenseParty(CaptainTeam), BattleTeam::new(
        defender.captain_team().clone(), DefenseParty(CaptainTeam),
    ));

    if let Some(team) = attacker.second_team() {
        teams.insert(AttackParty(SecondTeam), BattleTeam::new(
            team.clone(), AttackParty(SecondTeam),
        ));
    }
    if let Some(team) = attacker.third_team() {
        teams.insert(AttackParty(ThirdTeam), BattleTeam::new(
            team.clone(), AttackParty(ThirdTeam),
        ));
    }

    if let Some(team) = defender.second_team() {
        teams.insert(DefenseParty(SecondTeam), BattleTeam::new(
            team.clone(), DefenseParty(SecondTeam),
        ));
    }
    if let Some(team) = defender.third_team() {
        teams.insert(DefenseParty(ThirdTeam), BattleTeam::new(
            team.clone(), DefenseParty(ThirdTeam),
        ));
    }


    teams
}

fn play_turn(state: &mut HashMap<ShiaiPosition, BattleTeam>, subject: ShiaiPosition) {
    if let Some(team) = state.get(&subject) {
        if team.is_alive() {
            // combo skills part
            // active skills part
            attack(state, subject);
        }
    }
}


fn select_turn_team(turn: u8) -> ShiaiPosition {
    match turn % 6 {
        1 => AttackParty(CaptainTeam),
        2 => DefenseParty(CaptainTeam),
        3 => AttackParty(SecondTeam),
        4 => DefenseParty(SecondTeam),
        5 => AttackParty(ThirdTeam),
        0 => DefenseParty(ThirdTeam),
        _ => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use crate::data::{stub_party, stub_party_2};
    use super::*;

    #[test]
    fn test_start_state() {
        let attacker = stub_party();
        let defender = stub_party_2();

        let state = start_state(attacker.clone(), defender.clone());
        assert_eq!(
            &AttackParty(CaptainTeam),
            state.get(&AttackParty(CaptainTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.captain_team(),
            state.get(&AttackParty(CaptainTeam)).unwrap().original_team()
        );
        assert_eq!(
            &AttackParty(SecondTeam),
            state.get(&AttackParty(SecondTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.second_team().unwrap(),
            state.get(&AttackParty(SecondTeam)).unwrap().original_team()
        );
        assert_eq!(
            &AttackParty(ThirdTeam),
            state.get(&AttackParty(ThirdTeam)).unwrap().position()
        );
        assert_eq!(
            attacker.third_team().unwrap(),
            state.get(&AttackParty(ThirdTeam)).unwrap().original_team()
        );

        assert_eq!(
            &DefenseParty(CaptainTeam),
            state.get(&DefenseParty(CaptainTeam)).unwrap().position()
        );
        assert_eq!(
            defender.captain_team(),
            state.get(&DefenseParty(CaptainTeam)).unwrap().original_team()
        );
        assert_eq!(
            &DefenseParty(SecondTeam),
            state.get(&DefenseParty(SecondTeam)).unwrap().position()
        );
        assert_eq!(
            defender.second_team().unwrap(),
            state.get(&DefenseParty(SecondTeam)).unwrap().original_team()
        );
        assert_eq!(
            &DefenseParty(ThirdTeam),
            state.get(&DefenseParty(ThirdTeam)).unwrap().position()
        );
        assert_eq!(
            defender.third_team().unwrap(),
            state.get(&DefenseParty(ThirdTeam)).unwrap().original_team()
        );
    }
    #[test]
    fn test_select_active_team() {
        let attacker = stub_party();
        let defender = stub_party();
        let selected_team = select_turn_team(35);

        assert_eq!(AttackParty(ThirdTeam), selected_team);
    }
}