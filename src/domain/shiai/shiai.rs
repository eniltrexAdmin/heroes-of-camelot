use crate::domain::*;
use crate::domain::shiai::print_shiai::print_shiai;
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
    pub state_result: ShiaiState
}


pub struct Shiai{
    pub current_state: ShiaiState,
    pub result: Vec<ShiaiTurn>,
}

impl Shiai{
    pub fn new(attacker: Party, defender: Party) -> Self {
        Self{
            current_state: ShiaiState::new(attacker, defender),
            result: vec![]
        }
    }

    pub fn battle(mut self) -> Self{
        let mut turn = 1;
        while turn < 10 {
            let active_team = ShiaiPosition::active_team(turn);
            self = self.play_turn(active_team);
            turn = turn + 1;
        }
        print_shiai(&self);
        self
    }

    fn play_turn(self, subject: ShiaiPosition) -> Self {
        match self.current_state.state.get(&subject) {
            Some(team) if team.is_alive() => team,
            _ => return self,
        };

        let mut actions = vec![];

        // combo skills
        // active skills

        //attack
        let (new_state, action) = match attack_action(self.current_state.clone(), subject.clone()) {
            Ok((state, action)) => (state, action),
            Err(error) => panic!("Error while applying shiai: {:?}", error),
        };
        actions.push(action);

        // Finally construct turn played.

        let mut turns = self.result.clone();
        let turn_played = ShiaiTurn{
            subject: subject.clone(),
            actions,
            state_result: new_state.clone(),
        };
        turns.push(turn_played);

        Self{
            current_state: new_state,
            result: turns,
        }
    }
}



