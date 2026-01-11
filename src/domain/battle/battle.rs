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
    {
        let team_alive = current_state
            .state
            .get(&subject)
            .map(|team| team.is_alive())
            .unwrap_or(false);

        if !team_alive {
            return None;
        }
    }

    let mut turn_events = vec![];


    // --- Perform skill ---
    {
        // Take a mutable borrow of the team to perform skill
        // Short-lived borrow: the scope ends after this block
        if let Some(team) = current_state.state.get(&subject) {
            let events = team.perform_skill(&current_state); // pass current_state immutably
            turn_events.extend(events.clone());
            current_state.apply_domain_events(events); // mutable borrow allowed here
        }
    }

    // --- Perform attack ---
    {
        // Another short-lived mutable borrow
        if let Some(team) = current_state.state.get(&subject) {
            let events = team.attack(&current_state); // again, current_state immutably
            turn_events.extend(events.clone());
            current_state.apply_domain_events(events);
        }
    }

    Some(TurnLog {
        subject,
        events: turn_events,
        state_result: current_state.clone(),
    })
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