use crate::domain::*;
use crate::domain::shiai::print_shiai::{print_shiai, print_shiai_turn};
use super::*;

#[derive(Debug)]
pub enum ShiaiError{
    SubjectMissingError,
    TargetMissingError
}

#[derive(Clone)]
pub struct ShiaiTurn{
    pub subject: ShiaiPosition,
    pub actions: Vec<ShiaiAction>,
    pub state_result: ShiaiState,
    pub number: usize,
}


pub struct Shiai{
    pub init_state: ShiaiState,
    pub result: Vec<ShiaiTurn>,
}

impl Shiai{
    pub fn new(attacker: Party, defender: Party) -> Self {
        let init_state = ShiaiState::new(attacker, defender);
        Self{
            init_state: init_state.clone(),
            result: vec![]
        }
    }

    pub fn battle(mut self) -> Self{
        let mut turn = 1;
        while turn < 29 {
            self = self.play_turn(turn);
            turn = turn + 1;

        }
        print_shiai(&self);
        self
    }

    fn play_turn(self, turn_number: usize) -> Self {
        println!("Trying to playing Turn #{}", turn_number);
        let current_state = match self.result.len() {
            0 => self.init_state.clone(),
            _ => self.result.get(self.result.len()-1)
                    .map(|t| t.state_result.clone())
                    .unwrap_or(self.init_state.clone())
        };

        let subject= ShiaiPosition::active_team(turn_number as u8);
        let team = match current_state.get(&subject) {
            Some(team) => team,
            None => return self,
        };
        // for now not adding turn if team is dead.
        if !team.is_alive() {
            return self;
        }

        let mut actions = vec![];
        let mut new_state = current_state.clone();

        // combo skills
        // active skills

        //attack
        match attack_action(current_state, subject.clone()) {
            Ok((state, action)) => {
                new_state = state;
                actions.push(action);
            }
            Err(error) => panic!("Error while applying shiai: {:?}", error),
        };



        // Finally construct turn played.

        let mut turns = self.result.clone();
        let turn_played = ShiaiTurn{
            subject: subject.clone(),
            actions,
            state_result: new_state,
            number: turn_number
        };
        // print_shiai_turn(&turn_played);
        turns.push(turn_played);

        Self{
            init_state: self.init_state,
            result: turns,
        }
    }
}



