use crate::domain::*;
use crate::domain::shiai::shiai_state::ShiaiEvent;
use super::*;

#[derive(Debug)]
pub enum ShiaiError{
    SubjectMissingError,
    TargetMissingError
}

#[derive(Clone, Debug)]
pub struct TurnLog {
    pub subject: ShiaiPosition,
    pub events: Vec<ShiaiEvent>,
    pub state_result: ShiaiState,
}


pub struct ShiaiResult {
    pub init_state: ShiaiState,
    pub turn_logs: Vec<TurnLog>,
}

impl ShiaiResult {
    pub fn new(attacker: Party, defender: Party) -> Self {
        let init_state = ShiaiState::new(attacker, defender);


        Self{
            init_state: init_state.clone(),
            turn_logs: battle(init_state)
        }
    }
}

pub fn battle(init_state: ShiaiState) -> Vec<TurnLog>{
    let mut turn_logs = vec![];
    let mut turn_number = 1;
    let mut current_state = init_state;
    loop {
        if winner(&current_state).is_some() || turn_number > 100 {
            break;
        }
        let subject= ShiaiPosition::active_team(turn_number as u8);
        match  play_turn(current_state.clone(), subject) {
            Some(turn_played) => {
                current_state = turn_played.state_result.clone();
                turn_logs.push(turn_played);
            },
            None => {
                println!("No turn played as team is not present");
            }
        }
        turn_number = turn_number + 1;
    }
    turn_logs
}

fn play_turn(current_state: ShiaiState, subject: ShiaiPosition) -> Option<TurnLog> {
    let mut turn_events = vec![];
    let mut new_state = current_state.clone();

    let team = match current_state.get(&subject) {
        Some(team) => team,
        None => return None,
    };

    // combo skills
    // active skills
    // we can pre calculate some stuff, and set it in "team"., but I will try without it at first.

    //attack
    if team.is_alive() {
        // so far no errors now.
        let (next_state, event) = attack(current_state, subject.clone());
        new_state = next_state;
        turn_events.extend(event);
    }

    Some(TurnLog {
        subject: subject.clone(),
        events: turn_events,
        state_result: new_state,
    })
}

// TODO test.
fn winner(current_state: &ShiaiState) -> Option<PartyPosition> {
    let defense_alive = [
        DefenseParty(CaptainTeam),
        DefenseParty(SecondTeam),
        DefenseParty(ThirdTeam)
    ]
        .iter()
        .any(|pos| current_state.get(&pos)
            .map_or(false, |team| team.is_alive()));

    let attack_alive = [
        AttackParty(CaptainTeam),
        AttackParty(SecondTeam),
        AttackParty(ThirdTeam)
    ]
        .iter()
        .any(|pos| current_state.get(&pos)
            .map_or(false, |team| team.is_alive()));

    match (attack_alive, defense_alive) {
        (true, false) => Some(PartyPosition::Attack),
        (false, true) => Some(PartyPosition::Defense),
        (false, false) => Some(PartyPosition::Defense),
        (true, true) => None,   // No winner yet
    }
}