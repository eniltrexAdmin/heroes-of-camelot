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
        while turn < 30 {
            self = self.play_turn();
            turn = turn + 1;

        }
        // print_shiai(&self);
        self
    }

    fn play_turn(self) -> Self {
        let turn = self.result.len();


        let current_state = match turn {
            0 => self.init_state.clone(),
            _ => self.result.get(turn-1)
                    .map(|t| t.state_result.clone())
                    .unwrap_or(self.init_state.clone())
        };

        let subject= ShiaiPosition::active_team(turn as u8 + 1);
        match current_state.get(&subject) {
            Some(team) if team.is_alive() => team,
            _ => return self,
        };

        // println!("Playing Turn #{}", turn +1);


        let mut actions = vec![];

        // combo skills
        // active skills

        //attack
        let (new_state, action) = match attack_action(current_state, subject.clone()) {
            Ok((state, action)) => (state, action),
            Err(error) => panic!("Error while applying shiai: {:?}", error),
        };
        actions.push(action);

        // Finally construct turn played.

        let mut turns = self.result.clone();
        let turn_played = ShiaiTurn{
            subject: subject.clone(),
            actions,
            state_result: new_state,
            number: turn + 1
        };
        // print_shiai_turn(&turn_played);
        turns.push(turn_played);

        Self{
            init_state: self.init_state,
            result: turns,
        }
    }
}



