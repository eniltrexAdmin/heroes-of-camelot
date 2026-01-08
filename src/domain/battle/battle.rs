use crate::domain::*;
use super::*;

#[derive(Debug)]
pub enum BattleError {
    SubjectMissingError,
    TargetMissingError
}

#[derive(Clone, Debug)]
pub struct TurnLog {
    pub subject: BattlePosition,
    pub events: Vec<BattleEvent>,
    pub state_result: BattleState,
}


pub struct BattleResult {
    pub init_state: BattleState,
    pub turn_logs: Vec<TurnLog>,
}

impl BattleResult {
    pub fn new(attacker: Party, defender: Party) -> Self {
        let init_state = BattleState::new(attacker, defender);


        Self{
            init_state: init_state.clone(),
            turn_logs: battle(init_state)
        }
    }
}

pub fn battle(init_state: BattleState) -> Vec<TurnLog>{
    let mut turn_logs = vec![];
    let mut turn_number = 1;
    let mut current_state = init_state;
    loop {
        if winner(&current_state).is_some() || turn_number > 100 {
            break;
        }
        let subject= BattlePosition::active_team(turn_number as u8);
        match  play_turn(&mut current_state, subject) {
            Some(turn_played) => {
                current_state = turn_played.state_result.clone();
                turn_logs.push(turn_played);
            },
            None => {
                println!("Nothing was played");
            }
        }
        turn_number = turn_number + 1;
    }
    turn_logs
}

fn play_turn(current_state: &mut BattleState, subject: BattlePosition) -> Option<TurnLog> {
    let mut turn_events = vec![];

    let team = match current_state.get(&subject) {
        Some(team) => team,
        None => return None,
    };

    //attack
    if team.is_alive() {

        // combo skills
        // active skills
        // we can pre calculate some stuff, and set it in "team"., but I will try without it at first.

        // so far no errors now.
        let events = team.attack(&current_state);
        turn_events.extend(events.clone());

        current_state.apply_domain_events(events);

        return Some(TurnLog {
            subject: subject.clone(),
            events: turn_events,
            state_result: current_state.clone(),
        })
    }
    None
}

// TODO test.
fn winner(current_state: &BattleState) -> Option<PartyPosition> {
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